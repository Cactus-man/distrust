use std::error::Error;
use std::fmt::{Debug, Display};
use websockets::WebSocketError;

#[derive(Debug)]
pub enum GatewayError {
    ResumeError,
    DisconnectError(u16, String),
    NetworkError(String),
    InitializationError,
}

impl From<WebSocketError> for GatewayError {
    fn from(_: WebSocketError) -> Self {
        todo!()
    }
}

impl From<serde_json::Error> for GatewayError {
    fn from(_: serde_json::Error) -> Self {
        todo!()
    }
}

impl Display for GatewayError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for GatewayError {}
