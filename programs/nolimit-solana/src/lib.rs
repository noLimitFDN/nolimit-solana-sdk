//! # noLimit Solana SDK
//!
//! Privacy-first AI, Swap, and Mixer utilities for Solana.
//!
//! ## Features
//!
//! - **Chat**: Uncensored AI with zero data retention
//! - **Swap**: Privacy-enhanced DEX via Jupiter
//! - **Mixer**: Break on-chain transaction links
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use nolimit_solana::NoLimitClient;
//! use solana_sdk::signer::keypair::Keypair;
//!
//! #[tokio::main]
//! async fn main() {
//!     let keypair = Keypair::new();
//!     let client = NoLimitClient::new(keypair, None).unwrap();
//!     
//!     let response = client.chat().send("Hello").await.unwrap();
//!     println!("{}", response.message);
//! }
//! ```

pub mod client;
pub mod chat;
pub mod swap;
pub mod mixer;
pub mod error;
pub mod types;

pub use client::NoLimitClient;
pub use chat::{ChatClient, ChatOptions, ChatResponse};
pub use swap::{SwapClient, SwapParams, SwapQuote, SwapResult};
pub use mixer::{MixerClient, MixParams, MixResult, MixStatus};
pub use error::{NoLimitError, Result};
pub use types::*;

/// Default server URL
pub const DEFAULT_SERVER: &str = "https://x402.nolimit.foundation";

/// Token mint addresses
pub mod tokens {
    pub const SOL: &str = "So11111111111111111111111111111111111111112";
    pub const USDC: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
    pub const USDT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
}

/// Service pricing (in USDC)
pub mod pricing {
    pub const CHAT: f64 = 0.05;
    pub const SWAP: f64 = 0.10;
    pub const MIXER_BASE: f64 = 0.075;
    pub const MIXER_PERCENT: f64 = 1.0;
}

