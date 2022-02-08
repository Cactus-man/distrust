mod events;
mod shards;

use crate::{
    snowflakes::Snowflake,
    types::{
        applications::PartialApplication,
        channels::{threads::ThreadMember, Channel},
        guilds::{Guild, UnavailableGuild},
        messages::{emojis::Emoji, stickers::Sticker},
        users::User,
    },
};
use chrono::{DateTime, Utc};
pub use events::GatewayDispatch;
pub use events::Intent;
use serde::{Deserialize, Serialize};
pub use shards::Shard;

#[derive(Serialize, Deserialize)]
pub struct GatewayPayload {
    op: u8,
    d: PayloadData,
    s: Option<u64>,
    t: Option<String>, // ? Change to GatewayDispatch
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayloadData {
    Hello {
        heartbeat_interval: u64,
    },
    Ready {
        v: u8,
        user: User,
        guilds: Vec<UnavailableGuild>,
        session_id: String,
        shard: Option<(u16, u16)>,
        application: PartialApplication,
    },
    Null(()),
    Bool(bool),
    ThreadListSync {
        guild_id: Snowflake,
        channel_ids: Option<Vec<Snowflake>>,
        threads: Vec<Channel>,
        members: Vec<ThreadMember>,
    },
    Channel(Channel),
    ThreadMemberUpdate {
        id: Snowflake,
        guild_id: Snowflake,
        member_count: u8,
        added_members: Option<Vec<ThreadMember>>,
        removed_member_ids: Option<Vec<Snowflake>>,
    },
    ChannelPinsUpdate {
        guild_id: Snowflake,
        channel_id: Snowflake,
        last_pin_timestamp: Option<DateTime<Utc>>,
    },
    Guild(Guild),
    GuildBan {
        guild_id: Snowflake,
        user: User,
    },
    GuildEmojiUpdate {
        guild_id: Snowflake,
        emojis: Vec<Emoji>,
    },
    GuildStickerUpdate {
        guild_id: Snowflake,
        stickers: Vec<Sticker>,
    },
    GuildIntegrationsUpdate {
        guild_id: Snowflake,
    },
}
