use serde::{Deserialize, Serialize};

use crate::snowflakes::Snowflake;
use crate::types::users::User;

pub mod stickers {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]

    pub struct Sticker {
        id: Snowflake,
        pack_id: Option<Snowflake>,
        name: String,
        description: Option<String>,
        tags: Option<String>,
        m_type: Option<StickerType>,
        format_type: StickerFormat,
        available: Option<bool>,
        guild_id: Option<Snowflake>,
        user: Option<User>,
        sort_value: Option<i32>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    enum StickerType {
        STANDARD = 1,
        GUILD = 2,
    }

    #[derive(Debug, Serialize, Deserialize)]

    pub enum StickerFormat {
        PNG = 1,
        APNG = 2,
        LOTTIE = 3,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StickerPack {
        id: Snowflake,
        name: String,
        sku_id: Snowflake,
        cover_sticker_id: Option<Snowflake>,
        description: String,
        stickers: Vec<Sticker>,
    }
}

pub mod emojis {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Emoji {
        id: Snowflake,
    }
}
