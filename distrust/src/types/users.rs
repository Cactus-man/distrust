use serde::{Serialize, Deserialize};

use crate::snowflakes::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Snowflake,
}