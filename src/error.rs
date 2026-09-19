use thiserror::Error;

#[derive(Error, Debug)]
pub enum GliaError {
    #[error("WebSocket connection failed: {0}")]
    ConnectionError(String),

    #[error("Serialization/Deserialization error: {0}")]
    CodecError(#[from] serde_json::Error),

    #[error("Channel join rejected: {0}")]
    JoinRejected(String),

    #[error("Agent execution error: {0}")]
    ExecutionError(String),

    #[error("Channel or agent timed out")]
    Timeout,

    #[error("Invalid URL: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("Client is not connected")]
    NotConnected,
}
