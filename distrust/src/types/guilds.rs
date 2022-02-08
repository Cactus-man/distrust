use serde::{Deserialize, Serialize};

use crate::snowflakes::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnavailableGuild {
    id: Snowflake,
    unavailable: bool
}
