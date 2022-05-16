#![feature(async_closure)]

use std::env;
use std::error;
use std::fs;
use std::path::Path;

use anyhow::Context;
use futures::stream::{self, StreamExt};
use futures::FutureExt;
use log::{info, warn};
use simple_logger::SimpleLogger;
use tokio;

mod api;

const N_WORKERS: usize = 2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    SimpleLogger::new().init().unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let dst_dir = env::args().nth(1).unwrap_or("books".to_string());
    info!(
        "creating distance dir {:?} in {:?}",
        &dst_dir,
        env::current_dir()?
    );
    fs::create_dir_all(&dst_dir).context("creating target dir")?;

    let config = api::Config {
        connect_timeout: None,
        fetch_timeout: None,
        addr: env::var("KY_SERVER")
            .context("KyBook 3 content server address. Example: http://192.168.1.62:8080")?,
        username: env::var("KY_USER").context("KyBook 3 content server username")?,
        password: env::var("KY_PASSW").context("KyBook 3 content server password")?,
    };
    let client = config.build_client().context("creating HTTP client")?;

    let all_books = client.fetch_inbox().await.context("fetching book list")?;
    info!("got {:?} books", all_books.len());

    let to_download = all_books.iter().map(|book| {
        info!("fetching {}", &book.name);
        client
            .fetch_book(&book.path)
            .then(async move |response| (book, response))
    });
    // start downloading with N_WORKERS downloading tasks in parallel
    let mut downloaded = stream::iter(to_download)
        .buffer_unordered(N_WORKERS)
        .enumerate();

    while let Some((i, (book, response))) = downloaded.next().await {
        let content = match response {
            Err(err) => {
                warn!("downloading book {}: {}", &book.name, err);
                continue;
            }
            Ok(content) => content,
        };
        let filename = Path::new(&dst_dir).join(&book.name);
        info!("writing {:?}", &filename);
        fs::write(&filename, content).context(format!("writing {:?}", &filename))?;
        info!("got {}/{} books", i + 1, all_books.len());
    }
    Ok(())
}
