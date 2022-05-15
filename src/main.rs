use std::env;
use std::error;

mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let config = api::Config {
        addr: "http://192.168.1.62:8080/list".to_string(),
        username: env::var("KY_USER")?,
        password: env::var("KY_PASSW")?,
    };
    let client = config.build_client()?;

    println!("{:?}", client.fetch_inbox().await?);
    Ok(())
}
