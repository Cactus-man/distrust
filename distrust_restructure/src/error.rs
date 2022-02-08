use std::fmt::Display;

use gateway::GatewayError;
use discord_rest::RestError;

#[derive(Debug)]
pub enum Error {
    GatewayError(GatewayError),
    RequestFailedError { reason: String, cause: RestError },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<RestError> for Error {
    fn from(_: RestError) -> Self {
        todo!()
    }
}

impl From<GatewayError> for Error {
    fn from(_: GatewayError) -> Self {
        todo!()
    }
}
