use glia_sdk::{GliaEvent, GliaOptions, PhoenixFrame};
use serde_json::json;

#[test]
fn test_phoenix_frame_serialization() {
    let frame = PhoenixFrame::join("session:demo_app:user_123", "1");
    let json_str = frame.to_json().expect("Frame must serialize");

    assert_eq!(
        json_str,
        r#"["1","1","session:demo_app:user_123","phx_join",{}]"#
    );
}

#[test]
fn test_phoenix_frame_deserialization() {
    let raw = r#"["1","1","session:demo_app:user_123","phx_reply",{"response":{},"status":"ok"}]"#;
    let frame = PhoenixFrame::from_json(raw).expect("Frame must deserialize");

    assert_eq!(frame.join_ref, Some("1".to_string()));
    assert_eq!(frame.reference, Some("1".to_string()));
    assert_eq!(frame.topic, "session:demo_app:user_123");
    assert_eq!(frame.event, "phx_reply");
    assert_eq!(frame.payload.get("status").unwrap(), "ok");
}

#[test]
fn test_heartbeat_frame() {
    let hb = PhoenixFrame::heartbeat("42");
    let json_str = hb.to_json().unwrap();
    assert_eq!(json_str, r#"[null,"42","phoenix","heartbeat",{}]"#);
}

#[test]
fn test_event_parsing() {
    let thinking_payload = json!({"content": "Analyzing Kotlin AST..."});
    let event = GliaEvent::parse("thinking_delta", &thinking_payload).unwrap();
    assert_eq!(
        event,
        GliaEvent::ThinkingDelta {
            content: "Analyzing Kotlin AST...".to_string()
        }
    );

    let done_payload = json!({"text": "package cl.zea\nclass SecureClient"});
    let event = GliaEvent::parse("done", &done_payload).unwrap();
    assert_eq!(
        event,
        GliaEvent::Done {
            text: "package cl.zea\nclass SecureClient".to_string()
        }
    );
}

#[test]
fn test_glia_options_url_builder() {
    let options = GliaOptions::new("ws://localhost:4003/socket/websocket", "mobile", "dev")
        .with_token("test_jwt_123");

    let url = options.build_url().unwrap();
    assert_eq!(url.scheme(), "ws");
    assert_eq!(url.host_str(), Some("localhost"));
    assert_eq!(url.port(), Some(4003));
    assert_eq!(url.path(), "/socket/websocket");

    let query: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
    assert_eq!(query.get("vsn").unwrap(), "2.0.0");
    assert_eq!(query.get("token").unwrap(), "test_jwt_123");
    assert_eq!(options.session_topic(), "session:mobile:dev");
}
