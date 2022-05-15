use std::env;
use std::error;
use std::fs;
use std::path::Path;

use anyhow::Context;
use tokio;

mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let dst_dir = env::args().nth(1).unwrap_or("books".to_string());
    println!(
        "creating distance dir {:?} in {:?}",
        &dst_dir,
        env::current_dir()?
    );
    fs::create_dir_all(&dst_dir).context("creating distance dir")?;

    let config = api::Config {
        connect_timeout: None,
        fetch_timeout: None,
        addr: env::var("KY_SERVER")?,
        username: env::var("KY_USER")?,
        password: env::var("KY_PASSW")?,
    };
    let client = config.build_client().context("creating HTTP client")?;

    let all_books = client.fetch_inbox().await.context("fetching book list")?;
    println!("got {:?} books", all_books.len());

    for book in all_books {
        println!("fetching {:?}", book);
        let content = client
            .fetch_book(&book.path)
            .await
            .context(format!("fetching {}", &book.name))?;

        let filename = Path::new(&dst_dir).join(&book.name);
        println!("writing {:?}", &filename);
        fs::write(&filename, content).context(format!("writing {:?}", &filename))?;
    }

    Ok(())
}
