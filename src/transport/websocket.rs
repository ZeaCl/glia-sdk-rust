use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
    MaybeTlsStream, WebSocketStream,
};
use crate::error::GliaError;
use crate::protocol::PhoenixFrame;

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct PhoenixTransport {
    sender: futures_util::stream::SplitSink<WsStream, Message>,
    receiver: futures_util::stream::SplitStream<WsStream>,
}

impl PhoenixTransport {
    /// Connects to a WebSocket URL and establishes the Phoenix transport.
    pub async fn connect(url: &url::Url) -> Result<Self, GliaError> {
        let (ws_stream, _) = connect_async(url.as_str())
            .await
            .map_err(|e| GliaError::ConnectionError(e.to_string()))?;

        let (sender, receiver) = ws_stream.split();
        Ok(Self { sender, receiver })
    }

    /// Sends a Phoenix frame over the WebSocket connection.
    pub async fn send(&mut self, frame: &PhoenixFrame) -> Result<(), GliaError> {
        let text = frame.to_json()?;
        self.sender
            .send(Message::Text(text))
            .await
            .map_err(|e| GliaError::ConnectionError(e.to_string()))
    }

    /// Receives the next Phoenix frame from the WebSocket connection.
    pub async fn next_frame(&mut self) -> Result<Option<PhoenixFrame>, GliaError> {
        while let Some(msg_result) = self.receiver.next().await {
            match msg_result {
                Ok(Message::Text(text)) => {
                    let frame = PhoenixFrame::from_json(&text)?;
                    return Ok(Some(frame));
                }
                Ok(Message::Close(_)) => {
                    return Ok(None);
                }
                Ok(Message::Ping(_)) => {
                    let _ = self.sender.send(Message::Pong(vec![])).await;
                }
                Err(e) => {
                    return Err(GliaError::ConnectionError(e.to_string()));
                }
                _ => {}
            }
        }
        Ok(None)
    }
}
