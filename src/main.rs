#![feature(async_closure)]

use std::collections::HashSet;
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

// Empirically selected value.
// Further increase in the number of threads does not speed up the download (in the case of my iPhone 11)
const N_WORKERS: usize = 4;

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

    let local_files = fs::read_dir(&dst_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().into_string().ok())
        .flatten();
    let local_file_filter = HashSet::<String>::from_iter(local_files);

    let not_present_books: Vec<_> = all_books
        .iter()
        .filter(|book| !local_file_filter.contains(&book.name))
        .collect();

    info!(
        "{} out of {} books are already here",
        all_books.len() - not_present_books.len(),
        all_books.len()
    );
    info!("fetching {} books", not_present_books.len());
    let to_download = not_present_books.iter().map(|book| {
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
        info!("got {}/{} books", i + 1, not_present_books.len());
    }
    Ok(())
}
