use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

use std::time::Duration;

const DEFAULT_TIMEOUT: Duration = Duration::new(10, 0);

pub type Books = Vec<Book>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    #[serde(rename = "path")]
    path: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "size")]
    size: i64,
}

#[derive(Clone)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub addr: String,
}

impl Config {
    pub fn build_client(&self) -> Result<Client, reqwest::Error> {
        let client = reqwest::Client::builder()
            .connect_timeout(DEFAULT_TIMEOUT)
            .timeout(DEFAULT_TIMEOUT)
            .user_agent("ky-sync")
            .build()?;

        return Ok(Client {
            config: self.clone(),
            client: client,
        });
    }
}

pub struct Client {
    config: Config,
    client: reqwest::Client,
}

impl Client {
    pub async fn fetch_inbox(&self) -> Result<Books, reqwest::Error> {
        let response = self
            .client
            .get(&self.config.addr)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .query(&[("path", "/Inbox/")])
            .send()
            .await?;
        let data = response.bytes().await?;
        dbg!("{:?}", &data);
        let books = serde_json::from_slice::<Books>(&data).unwrap();
        Ok(books)
    }
}
