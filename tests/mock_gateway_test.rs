use futures_util::{SinkExt, StreamExt};
use glia_sdk::{GliaClient, GliaOptions, PhoenixFrame};
use serde_json::json;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::test]
async fn test_glia_client_end_to_end_mock_session() {
    // 1. Start a local mock Phoenix WebSocket server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let server_handle = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut ws = tokio_tungstenite::accept_async(stream).await.unwrap();

        // 1. Receive phx_join
        let join_msg = ws.next().await.unwrap().unwrap();
        let join_frame = PhoenixFrame::from_json(&join_msg.into_text().unwrap()).unwrap();
        assert_eq!(join_frame.event, "phx_join");

        // Reply phx_reply: status ok
        let reply = PhoenixFrame::new(
            join_frame.join_ref,
            join_frame.reference,
            join_frame.topic.clone(),
            "phx_reply",
            json!({"status": "ok", "response": {}}),
        );
        ws.send(Message::Text(reply.to_json().unwrap())).await.unwrap();

        // 2. Receive run
        let run_msg = ws.next().await.unwrap().unwrap();
        let run_frame = PhoenixFrame::from_json(&run_msg.into_text().unwrap()).unwrap();
        assert_eq!(run_frame.event, "run");

        // Emit thinking_delta
        let delta = PhoenixFrame::new(
            None,
            None,
            run_frame.topic.clone(),
            "thinking_delta",
            json!({"content": "Analyzing security vulnerability in Kotlin..."}),
        );
        ws.send(Message::Text(delta.to_json().unwrap())).await.unwrap();

        // Emit done
        let done = PhoenixFrame::new(
            None,
            None,
            run_frame.topic,
            "done",
            json!({"text": "// Secure refactored code\nclass SecureGliaOptions"}),
        );
        ws.send(Message::Text(done.to_json().unwrap())).await.unwrap();
    });

    // 2. Connect GliaClient to the mock gateway
    let url = format!("ws://127.0.0.1:{}/socket/websocket", port);
    let options = GliaOptions::new(url, "security", "microglia");
    let mut client = GliaClient::new(options);

    client.connect().await.expect("Client must connect and join");

    let response = client
        .run("Fix OWASP M1 in GliaOptions.kt", Some("You are a security engineer"))
        .await
        .expect("Run must return done text");

    assert_eq!(response, "// Secure refactored code\nclass SecureGliaOptions");

    server_handle.await.unwrap();
}
