use crate::snowflakes::Snowflake;

pub mod users {
    use super::Snowflake;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        id: Snowflake,
    }
}

pub mod members {}

pub mod threads {
    use super::Snowflake;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ThreadMember {
        id: Snowflake,
    }
}
