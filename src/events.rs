use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Declarative Tool registration schema matching Glia's Webhook Tool Bridge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicTool {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

/// Request payload sent to Glia in the `"run"` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunPayload {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<DynamicTool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Real-time streaming events emitted by the Glia ReAct Agent runtime.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GliaEvent {
    /// Agent status transition (e.g., "thinking", "executing_tool", "idle")
    Status { status: String },

    /// Streaming chunk of inner reasoning / thought
    ThinkingDelta { content: String },

    /// Streaming chunk of user-facing markdown text
    MessageDelta { content: String },

    /// Agent decided to call a tool
    ToolCall { name: String, args: Value },

    /// Execution result of a tool call
    ToolResult { name: String, result: Value },

    /// Agent completed reasoning and returned final text
    Done { text: String },

    /// An error occurred during agent execution
    Error { message: String },
}

impl GliaEvent {
    /// Parses a raw Phoenix event and its payload into a strongly-typed `GliaEvent`.
    pub fn parse(event_name: &str, payload: &Value) -> Option<Self> {
        match event_name {
            "status" => Some(GliaEvent::Status {
                status: payload.get("status")?.as_str()?.to_string(),
            }),
            "thinking_delta" => Some(GliaEvent::ThinkingDelta {
                content: payload.get("content")?.as_str()?.to_string(),
            }),
            "message_delta" => Some(GliaEvent::MessageDelta {
                content: payload.get("content")?.as_str()?.to_string(),
            }),
            "tool_call" => Some(GliaEvent::ToolCall {
                name: payload.get("name")?.as_str()?.to_string(),
                args: payload.get("args").cloned().unwrap_or(Value::Null),
            }),
            "tool_result" => Some(GliaEvent::ToolResult {
                name: payload.get("name")?.as_str()?.to_string(),
                result: payload.get("result").cloned().unwrap_or(Value::Null),
            }),
            "done" => Some(GliaEvent::Done {
                text: payload.get("text").and_then(|t| t.as_str()).unwrap_or("").to_string(),
            }),
            "error" => Some(GliaEvent::Error {
                message: payload.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error").to_string(),
            }),
            _ => None,
        }
    }
}
