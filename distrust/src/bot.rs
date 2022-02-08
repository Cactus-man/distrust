use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::gateway::Shard;

pub struct Bot<'a> {
    token: String,
    client: reqwest::Client,
    _shards: (Vec<Shard<'a>>, u16),
}

impl Bot<'_> {
    pub fn new(token: &str) -> Bot {
        // FIX (unnecessary panic) use reqwest::Client::builder()
        let token = token.to_string();
        Bot {
            token,
            client: reqwest::Client::new(),
            _shards: (Vec::new(), 1),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let json = crate::get(&self.client, &self.token, "/gateway/bot").await?;
        let json: GatewayMetadata = serde_json::from_str(&json)?;

        let shard = Shard::new(&json.url);

        shard.connect().await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct GatewayMetadata {
    url: String,
    shards: u16,
    session_start_limit: SessionStartLimit,
}

#[derive(Serialize, Deserialize)]
struct SessionStartLimit {
    total: u16,
    remaining: u16,
    reset_after: u64,
    max_concurrency: u16,
}
