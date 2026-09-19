use std::collections::HashMap;

/// Configuration options for connecting to the Glia Agent Gateway.
#[derive(Debug, Clone)]
pub struct GliaOptions {
    /// Gateway WebSocket URL, e.g., "ws://localhost:4003/socket/websocket" or "wss://gateway.zea.cl/socket/websocket"
    pub gateway_url: String,

    /// Application or tenant identifier
    pub app_id: String,

    /// User or session owner identifier
    pub user_id: String,

    /// Optional authentication JWT token
    pub token: Option<String>,

    /// Protocol version (defaults to "2.0.0")
    pub vsn: String,

    /// Default system prompt for agent sessions
    pub default_system_prompt: Option<String>,

    /// Extra query parameters for the WebSocket handshake
    pub params: HashMap<String, String>,

    /// Heartbeat interval in seconds (defaults to 30)
    pub heartbeat_secs: u64,
}

impl Default for GliaOptions {
    fn default() -> Self {
        Self {
            gateway_url: "ws://localhost:4003/socket/websocket".to_string(),
            app_id: "default".to_string(),
            user_id: "system".to_string(),
            token: None,
            vsn: "2.0.0".to_string(),
            default_system_prompt: None,
            params: HashMap::new(),
            heartbeat_secs: 30,
        }
    }
}

impl GliaOptions {
    pub fn new(gateway_url: impl Into<String>, app_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            gateway_url: gateway_url.into(),
            app_id: app_id.into(),
            user_id: user_id.into(),
            ..Default::default()
        }
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.default_system_prompt = Some(prompt.into());
        self
    }

    /// Computes the canonical topic for the session: `session:<app_id>:<user_id>`
    pub fn session_topic(&self) -> String {
        format!("session:{}:{}", self.app_id, self.user_id)
    }

    /// Builds the complete WebSocket URL including vsn and auth token in query parameters
    pub fn build_url(&self) -> Result<url::Url, url::ParseError> {
        let mut u = url::Url::parse(&self.gateway_url)?;
        u.query_pairs_mut().append_pair("vsn", &self.vsn);

        if let Some(ref t) = self.token {
            u.query_pairs_mut().append_pair("token", t);
        }

        for (k, v) in &self.params {
            u.query_pairs_mut().append_pair(k, v);
        }

        Ok(u)
    }
}
