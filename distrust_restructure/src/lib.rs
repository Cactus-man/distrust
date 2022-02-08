extern crate discord_rest;
extern crate gateway;
extern crate typing;

mod commands;
mod error;

pub use bot::{Bot, Config};
pub use gateway::builder::Intent;

mod bot {
    use crate::error::Error;
    use discord_rest::CachedClient;
    use gateway::{builder::Intent, Gateway};
    use typing::User;

    pub struct Config {
        shard: Option<(u32, u32)>,
        intents: Vec<Intent>,
    }

    impl Config {
        pub fn new() -> Self {
            Self {
                shard: None,
                intents: vec![Intent::Guilds],
            }
        }
    }

    #[derive(Debug)]
    pub struct Bot<'a> {
        token: String,
        me: Option<User>,
        client: CachedClient,
        gateway: Gateway<'a>,
    }

    impl<'a> Bot<'a> {
        pub fn new(token: String, config: Config) -> Result<Bot<'a>, Error> {
            let client = CachedClient::new()?;
            let gateway = Gateway::builder()
                .intents(config.intents.into_iter())
                .connect()?;
            Ok(Self {
                token,
                me: None,
                client,
                gateway,
            })
        }

        pub fn sharded(token: String, intents: Vec<Intent>) -> Result<Bot<'a>, Error> {
            let gateway = Gateway::builder()
                .intents(intents.into_iter())
                .connect()?;
            Ok(Self {
                token,
                me: None,
                client: CachedClient::new()?,
                gateway,
            })
        }

        pub async fn me(&mut self) -> Result<&User, Error> {
            if let None = self.me {
                let user: User = self.client.get("/users/@me", &self.token).await?;
                self.me = Some(user);
            }
            Ok(self.me.as_ref().unwrap())
        }
    }
}
