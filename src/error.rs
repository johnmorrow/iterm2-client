use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Protobuf decode error: {0}")]
    Decode(#[from] prost::DecodeError),

    #[error("API error from server: {0}")]
    Api(String),

    #[error("Non-OK status: {0}")]
    Status(String),

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Request timed out after {0:?}")]
    Timeout(std::time::Duration),

    #[error("Unexpected response: expected {expected}, got different submessage")]
    UnexpectedResponse { expected: &'static str },
}

pub type Result<T> = std::result::Result<T, Error>;
