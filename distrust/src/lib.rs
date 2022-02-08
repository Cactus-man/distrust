pub mod bot;

mod error;
mod gateway;
mod types;

pub use bot::Bot;
pub use error::DiscordError;
pub use gateway::Intent;

mod snowflakes {
    pub type Snowflake = String;
}

const URL: &str = "https://github.com/open-red-rainbow/harmony";
const VERSION: &str = "0.1.0-ALPHA";

async fn get(client: &reqwest::Client, token: &str, path: &str) -> Result<String, DiscordError> {
    let url = format!("https://discord.com/api/v9{}", path);
    let body = client
        .get(url)
        .header("Authorization", token)
        .header("User-Agent", format!("DiscordBot ({}, {})", URL, VERSION))
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
