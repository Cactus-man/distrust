use serde::{Deserialize, Serialize};

use crate::snowflakes::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    id: Snowflake,
}

pub mod threads {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Thread {}

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ThreadMember {}
}
