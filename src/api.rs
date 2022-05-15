use bytes;
use reqwest;
use serde::{Deserialize, Serialize};

use std::time::Duration;

const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_FETCH_TIMEOUT: Duration = Duration::from_secs(16 * 60);

pub type Books = Vec<Book>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    #[serde(rename = "path")]
    pub path: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "size")]
    pub size: i64,
}

#[derive(Clone)]
pub struct Config {
    pub connect_timeout: Option<Duration>,
    pub fetch_timeout: Option<Duration>,
    pub username: String,
    pub password: String,
    pub addr: String,
}

impl Config {
    pub fn build_client(&self) -> Result<Client, reqwest::Error> {
        let client = reqwest::Client::builder()
            .connect_timeout(self.connect_timeout.unwrap_or(DEFAULT_CONNECT_TIMEOUT))
            .timeout(self.fetch_timeout.unwrap_or(DEFAULT_FETCH_TIMEOUT))
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
        self.client
            .get(self.config.addr.clone() + "/list")
            .basic_auth(&self.config.username, Some(&self.config.password))
            .query(&[("path", "/Books/Inbox/")])
            .send()
            .await?
            .json::<Books>()
            .await
    }

    pub async fn fetch_book(&self, book_path: &str) -> Result<bytes::Bytes, reqwest::Error> {
        self.client
            .get(self.config.addr.clone() + "/download")
            .basic_auth(&self.config.username, Some(&self.config.password))
            .query(&[("path", book_path)])
            .send()
            .await?
            .bytes()
            .await
    }
}
