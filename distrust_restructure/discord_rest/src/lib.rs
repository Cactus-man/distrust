extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod errors;

use crate::errors::Error;
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;

pub use errors::Error as RestError;

const API_VERSION: u8 = 9;

#[derive(Debug)]
pub struct CachedClient {
    client: Client,
    agent: String,
}

impl CachedClient {
    pub fn new() -> Result<Self, Error> {
        let bu = Client::builder();
        let client = bu.build()?;

        let agent = format!(
            "DiscordBot ({}, {})",
            "https://github.com/open-red-rainbow/Harmony",
            env!("CARGO_PKG_VERSION"),
        );

        Ok(Self { client, agent })
    }

    #[inline(always)]
    pub async fn delete<'a, T>(&mut self, path: &str, token: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.request::<T>(path, token, Method::DELETE).await
    }

    #[inline(always)]
    pub async fn get<'a, T>(&mut self, path: &str, token: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.request::<T>(path, token, Method::GET).await
    }

    #[inline(always)]
    pub async fn patch<'a, T>(&mut self, path: &str, token: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.request::<T>(path, token, Method::PATCH).await
    }

    #[inline(always)]
    pub async fn post<'a, T>(&mut self, path: &str, token: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.request::<T>(path, token, Method::POST).await
    }

    async fn request<'de, T>(&mut self, path: &str, token: &str, method: Method) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("https://discord.com/api/v{}{}", API_VERSION, path);
        let res = self
            .client
            .request(method, url)
            .header("User-Agent", &self.agent)
            .header("Authorization", format!("Bot: {}", token))
            .send()
            .await?;
        let text = res.text().await?;
        let deserialized: T = serde_json::from_str(&text)?;
        Ok(deserialized)
    }
}
