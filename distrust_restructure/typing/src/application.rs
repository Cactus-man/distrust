use serde::{Deserialize, Serialize};
use crate::snowflakes::Snowflake;

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialApplication {
    id: Snowflake,
    flags: u16,
}
