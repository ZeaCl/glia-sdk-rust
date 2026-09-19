use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::timeout;

use crate::error::GliaError;
use crate::events::{DynamicTool, GliaEvent, RunPayload};
use crate::options::GliaOptions;
use crate::protocol::PhoenixFrame;
use crate::transport::PhoenixTransport;

pub struct GliaClient {
    options: GliaOptions,
    transport: Option<PhoenixTransport>,
    ref_counter: AtomicU64,
    topic: String,
}

impl GliaClient {
    /// Creates a new `GliaClient` with the given configuration options.
    pub fn new(options: GliaOptions) -> Self {
        let topic = options.session_topic();
        Self {
            options,
            transport: None,
            ref_counter: AtomicU64::new(1),
            topic,
        }
    }

    fn next_ref(&self) -> String {
        self.ref_counter.fetch_add(1, Ordering::SeqCst).to_string()
    }

    /// Establishes the WebSocket connection to the Glia Gateway and joins the session channel.
    pub async fn connect(&mut self) -> Result<(), GliaError> {
        let url = self.options.build_url()?;
        let mut transport = PhoenixTransport::connect(&url).await?;

        // 1. Join session topic
        let join_ref = self.next_ref();
        let join_frame = PhoenixFrame::join(&self.topic, &join_ref);
        transport.send(&join_frame).await?;

        // 2. Wait for join acknowledgement (phx_reply with status: "ok")
        let join_timeout = Duration::from_secs(10);
        let confirmed = timeout(join_timeout, async {
            while let Some(frame) = transport.next_frame().await? {
                if frame.topic == self.topic && frame.event == "phx_reply" {
                    if let Some(status) = frame.payload.get("status").and_then(|s| s.as_str()) {
                        if status == "ok" {
                            return Ok(());
                        } else {
                            return Err(GliaError::JoinRejected(format!(
                                "Join rejected with status '{}': {:?}",
                                status, frame.payload
                            )));
                        }
                    }
                }
            }
            Err(GliaError::ConnectionError("Connection closed during channel join".to_string()))
        })
        .await
        .map_err(|_| GliaError::Timeout)??;

        self.transport = Some(transport);
        Ok(confirmed)
    }

    /// Sends a prompt to the ReAct agent and returns an asynchronous stream of streaming deltas.
    pub async fn run_stream(
        &mut self,
        message: &str,
        system_prompt: Option<&str>,
        tools: Vec<DynamicTool>,
    ) -> Result<mpsc::Receiver<GliaEvent>, GliaError> {
        let push_ref = self.next_ref();
        let payload = RunPayload {
            message: message.to_string(),
            system_prompt: system_prompt
                .or(self.options.default_system_prompt.as_deref())
                .map(|s| s.to_string()),
            tools,
            provider: None,
            model: None,
        };

        let run_frame = PhoenixFrame::push(
            &self.topic,
            "run",
            &push_ref,
            serde_json::to_value(&payload)?,
        );

        let transport = self.transport.as_mut().ok_or(GliaError::NotConnected)?;
        transport.send(&run_frame).await?;

        let (tx, rx) = mpsc::channel(64);

        // Consume frames until "done" or "error"
        let topic = self.topic.clone();
        while let Some(frame) = transport.next_frame().await? {
            if frame.topic == topic {
                if let Some(event) = GliaEvent::parse(&frame.event, &frame.payload) {
                    let is_terminal = matches!(event, GliaEvent::Done { .. } | GliaEvent::Error { .. });
                    let _ = tx.send(event).await;

                    if is_terminal {
                        break;
                    }
                }
            }
        }

        Ok(rx)
    }

    /// Sends a prompt and waits for the agent to finish, returning the final text response.
    pub async fn run(
        &mut self,
        message: &str,
        system_prompt: Option<&str>,
    ) -> Result<String, GliaError> {
        let mut rx = self.run_stream(message, system_prompt, Vec::new()).await?;

        let mut final_text = String::new();
        while let Some(event) = rx.recv().await {
            match event {
                GliaEvent::Done { text } => {
                    final_text = text;
                    break;
                }
                GliaEvent::Error { message } => {
                    return Err(GliaError::ExecutionError(message));
                }
                _ => {}
            }
        }

        Ok(final_text)
    }
}
