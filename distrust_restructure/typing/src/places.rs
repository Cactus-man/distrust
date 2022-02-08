use crate::snowflakes::Snowflake;

pub mod guilds {
    use super::Snowflake;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Guild {
        id: Snowflake,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UnavailableGuild {
        id: Snowflake,
        unavailable: bool,
    }
}

pub mod channels {
    use super::Snowflake;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Channel {
        id: Snowflake,
    }

    pub mod threads {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Thread {}
    }
}
