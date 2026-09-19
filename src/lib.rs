//! # Glia SDK for Rust
//!
//! Official Rust Client SDK for the ZEA Glia ReAct Agent Platform.
//!
//! ## Overview
//!
//! Glia is the distributed, stateless ReAct agent gateway of the ZEA Platform.
//! This SDK provides an asynchronous, strongly-typed interface over Phoenix Channels v2,
//! matching the architecture of `glia-sdk-kotlin`, `glia-sdk-swift`, and `glia-sdk-ts`.
//!
//! ## Example
//!
//! ```no_run
//! use glia_sdk::{GliaClient, GliaOptions};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let options = GliaOptions::new("ws://localhost:4003/socket/websocket", "security", "microglia")
//!         .with_token("my_jwt_token")
//!         .with_system_prompt("You are an autonomous code remediation engineer.");
//!
//!     let mut client = GliaClient::new(options);
//!     client.connect().await?;
//!
//!     let response = client.run("Please refactor this insecure endpoint", None).await?;
//!     println!("Agent Response: {}", response);
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod events;
pub mod options;
pub mod protocol;
pub mod transport;

pub use client::GliaClient;
pub use error::GliaError;
pub use events::{DynamicTool, GliaEvent, RunPayload};
pub use options::GliaOptions;
pub use protocol::PhoenixFrame;
