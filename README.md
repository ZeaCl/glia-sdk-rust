# glia-sdk-rust 🦀

Official Rust Client SDK for the **ZEA Glia ReAct Agent Platform**.

Allows any Rust application, CLI or service (such as [ZEA Microglia](https://github.com/ZeaCl/microglia)) to communicate asynchronously with Glia cognitive agents via **Phoenix Channels v2** over WebSockets.

---

## Features

- **Phoenix Protocol v2**: Native serialization of 5-element frames `[join_ref, ref, topic, event, payload]`.
- **Async WebSocket Transport**: Built on `tokio` and `tokio-tungstenite`.
- **ReAct Streaming Events**: Real-time event handling (`status`, `thinking_delta`, `message_delta`, `tool_call`, `tool_result`, `done`, `error`).
- **Dynamic Tool Calling**: Register declarative schemas matching Glia Webhook Tool Bridge.
- **Resilient Channel Management**: Automatic join acknowledgement, reference counters, and heartbeat handling.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
glia-sdk = { path = "../glia-sdk-rust" }
# or when published to crates.io / git:
# glia-sdk = { git = "https://github.com/ZeaCl/glia-sdk-rust.git" }
```

---

## Quick Start

```rust
use glia_sdk::{GliaClient, GliaOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = GliaOptions::new(
        "wss://gateway.zea.cl/socket/websocket",
        "my_app",
        "user_123",
    )
    .with_token("JWT_AUTH_TOKEN")
    .with_system_prompt("You are a specialized security agent.");

    let mut client = GliaClient::new(options);
    client.connect().await?;

    // Streaming
    let mut rx = client.run_stream("Audit this code", None, vec![]).await?;
    while let Some(event) = rx.recv().await {
        println!("Event: {:?}", event);
    }

    // Direct aggregation
    let answer = client.run("Summarize the findings", None).await?;
    println!("Response: {}", answer);

    Ok(())
}
```

---

## Architecture & Ecosystem

| SDK | Platform | Repository |
| :--- | :--- | :--- |
| **Kotlin** | Android / JVM | [`ZeaCl/glia-sdk-kotlin`](https://github.com/ZeaCl/glia-sdk-kotlin) |
| **Swift** | iOS / macOS | [`ZeaCl/glia-sdk-swift`](https://github.com/ZeaCl/glia-sdk-swift) |
| **TypeScript** | Web / Node | [`ZeaCl/glia-sdk-ts`](https://github.com/ZeaCl/glia-sdk-ts) |
| **Rust** | CLI / Microglia / Core | [`ZeaCl/glia-sdk-rust`](https://github.com/ZeaCl/glia-sdk-rust) |

---

## Testing

```bash
cargo test
```
