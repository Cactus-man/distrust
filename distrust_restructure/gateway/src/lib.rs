mod error;
mod payloads;
mod sharding;
mod presence;

use crate::builder::GatewayBuilder;
use crate::sharding::shards::Shard;

pub use error::GatewayError;

#[derive(Debug)]
pub struct Gateway<'s> {
    shard: Shard<'s>,
}

impl<'s> Gateway<'s> {
    pub fn builder() -> crate::builder::GatewayBuilder {
        GatewayBuilder::new()
    }
}

pub mod builder {
    use crate::{Gateway, GatewayError};

    pub struct GatewayBuilder {
        intents: u32,
    }

    impl GatewayBuilder {
        pub fn new() -> Self {
            Self { intents: 0 }
        }

        pub fn intent(&mut self, intent: Intent) -> &mut Self {
            self.intents |= 1 << intent as u32;
            self
        }

        pub fn intents(&mut self, intents: impl Iterator<Item = Intent>) -> &mut Self {
            intents.for_each(|i| self.intents |= 1 << i as u32);
            self
        }

        pub fn shard(&mut self, shard: Option<(u32, u32)>) {
            match shard {
                Some((id, num)) => {
                    if id >= num {
                        panic!("Shard will not receive events")
                    };
                    todo!()
                }
                None => {
                    // no sharding
                    todo!()
                }
            }
        }

        pub fn connect<'a>(&mut self) -> Result<Gateway<'a>, GatewayError> {
            todo!()
        }
    }

    pub enum Intent {
        Guilds = 0,
        GuildMembers = 1,
        GuildBans = 2,
        GuildEmojisAndStickers = 3,
        GuildIntegrations = 4,
        GuildWebhooks = 5,
        GuildInvites = 6,
        GuildVoiceStates = 7,
        GuildPresences = 8,
        GuildMessages = 9,
        GuildMessageReactions = 10,
        GuildMessageTyping = 11,
        DirectMessages = 12,
        DirectMessageReactions = 13,
        DirectMessageTyping = 14,
        GuildScheduledEvents = 16,
    }
}
