use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// Phoenix Channel v2 protocol frame.
///
/// Encoded over the wire as a 5-element JSON array:
/// `[join_ref, ref, topic, event, payload]`
#[derive(Debug, Clone, PartialEq)]
pub struct PhoenixFrame {
    pub join_ref: Option<String>,
    pub reference: Option<String>,
    pub topic: String,
    pub event: String,
    pub payload: Value,
}

impl PhoenixFrame {
    pub fn new(
        join_ref: Option<String>,
        reference: Option<String>,
        topic: impl Into<String>,
        event: impl Into<String>,
        payload: Value,
    ) -> Self {
        Self {
            join_ref,
            reference,
            topic: topic.into(),
            event: event.into(),
            payload,
        }
    }

    /// Creates a `phx_join` frame to join a topic
    pub fn join(topic: &str, reference: &str) -> Self {
        Self::new(
            Some(reference.to_string()),
            Some(reference.to_string()),
            topic,
            "phx_join",
            serde_json::json!({}),
        )
    }

    /// Creates a `heartbeat` frame on the "phoenix" topic
    pub fn heartbeat(reference: &str) -> Self {
        Self::new(
            None,
            Some(reference.to_string()),
            "phoenix",
            "heartbeat",
            serde_json::json!({}),
        )
    }

    /// Creates a custom push frame on an active topic
    pub fn push(topic: &str, event: &str, reference: &str, payload: Value) -> Self {
        Self::new(
            None,
            Some(reference.to_string()),
            topic,
            event,
            payload,
        )
    }

    /// Serializes the frame into a JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserializes a frame from a JSON string
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

impl Serialize for PhoenixFrame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(5))?;
        seq.serialize_element(&self.join_ref)?;
        seq.serialize_element(&self.reference)?;
        seq.serialize_element(&self.topic)?;
        seq.serialize_element(&self.event)?;
        seq.serialize_element(&self.payload)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for PhoenixFrame {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr = Vec::<Value>::deserialize(deserializer)?;
        if arr.len() != 5 {
            return Err(serde::de::Error::custom(format!(
                "Expected 5-element Phoenix v2 array, got {} elements",
                arr.len()
            )));
        }

        let join_ref = arr[0].as_str().map(|s| s.to_string());
        let reference = arr[1].as_str().map(|s| s.to_string());
        let topic = arr[2].as_str().unwrap_or("").to_string();
        let event = arr[3].as_str().unwrap_or("").to_string();
        let payload = arr[4].clone();

        Ok(PhoenixFrame {
            join_ref,
            reference,
            topic,
            event,
            payload,
        })
    }
}
