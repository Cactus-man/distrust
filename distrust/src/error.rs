use std::error::Error;
use std::fmt::{Debug, Display};

use websockets::WebSocketError;

pub type Result<T> = std::result::Result<T, DiscordError>;

#[derive(Debug)]
pub enum DiscordError {
    // IDEA (String description) Add String description for more verbose error messages
    WebsocketError { source: WebSocketError },
    RESTApiError { source: reqwest::Error },
    JSONError { source: serde_json::Error },
    InternalError,
}

impl Display for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscordError::InternalError => Ok(()),
            DiscordError::WebsocketError { source } => write!(f, "WebsocketError ({})", source),
            DiscordError::RESTApiError { source } => write!(f, "WebsocketError ({})", source),
            DiscordError::JSONError { source } => write!(f, "WebsocketError ({})", source),
        }
    }
}

impl Error for DiscordError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DiscordError::WebsocketError { source } => Some(source),
            DiscordError::RESTApiError { source } => Some(source),
            DiscordError::JSONError { source } => Some(source),
            DiscordError::InternalError => None,
        }
    }
}

impl From<WebSocketError> for DiscordError {
    fn from(v: WebSocketError) -> Self {
        Self::WebsocketError { source: v }
    }
}

impl From<reqwest::Error> for DiscordError {
    fn from(v: reqwest::Error) -> Self {
        Self::RESTApiError { source: v }
    }
}

impl From<serde_json::Error> for DiscordError {
    fn from(v: serde_json::Error) -> Self {
        Self::JSONError { source: v }
    }
}

// enum GatewayClose {
//     UnknownError = 4000,
//     UnknownOpcode = 4001,
//     DecodeError = 4002,
//     NotAuthenticated = 4003,
//     AuthenticationFailed = 4004,
//     InvalidSeq = 4007,
//     RateLimited = 4008,
//     SessionTimedOut = 4009,
//     InvalidShard = 40010,
//     ShardingRequired = 4011,
//     InvalidAPIVersion = 4012,
//     InvalidIntents = 4013,
//     DisallowedIntents = 4014,
// }
