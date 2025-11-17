//! Mixer client

use crate::{NoLimitError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use solana_sdk::signer::{keypair::Keypair, Signer};
use std::sync::Arc;

/// Mix parameters
#[derive(Clone)]
pub struct MixParams {
    pub token: String,
    pub amount: String,
    pub recipient: String,
    pub delay_minutes: Option<u32>,
}

/// Mix result
#[derive(Debug, Clone)]
pub struct MixResult {
    pub mix_id: String,
    pub deposit_address: String,
    pub deposit_amount: String,
    pub fee: String,
    pub output_amount: String,
}

/// Mix status
#[derive(Debug, Clone, Deserialize)]
pub struct MixStatus {
    pub status: String,
    pub progress: u8,
    #[serde(rename = "currentHop")]
    pub current_hop: u32,
    #[serde(rename = "totalHops")]
    pub total_hops: u32,
    #[serde(rename = "completedAt")]
    pub completed_at: Option<String>,
    pub error: Option<String>,
}

#[derive(Serialize)]
struct MixRequest {
    token: String,
    amount: String,
    #[serde(rename = "recipientAddress")]
    recipient_address: String,
    #[serde(rename = "userAddress")]
    user_address: String,
    #[serde(rename = "delayMinutes")]
    delay_minutes: u32,
}

#[derive(Deserialize)]
struct MixApiResponse {
    #[serde(rename = "mixId")]
    mix_id: String,
    #[serde(rename = "depositAddress")]
    deposit_address: String,
    #[serde(rename = "depositAmount")]
    deposit_amount: String,
    fee: String,
    #[serde(rename = "outputAmount")]
    output_amount: String,
}

/// Mixer client
pub struct MixerClient {
    keypair: Arc<Keypair>,
    server_url: String,
    http: Client,
}

impl MixerClient {
    pub fn new(keypair: Arc<Keypair>, server_url: String) -> Self {
        Self {
            keypair,
            server_url,
            http: Client::new(),
        }
    }

    /// Create mix request
    pub async fn create(&self, params: MixParams) -> Result<MixResult> {
        // Requires x402 payment flow
        Err(NoLimitError::Unknown(
            "Mixer requires x402 payment - use TypeScript SDK for now".to_string(),
        ))
    }

    /// Get mix status
    pub async fn get_status(&self, mix_id: &str) -> Result<MixStatus> {
        let url = format!("{}/mixer/status/{}", self.server_url, mix_id);

        let response = self.http.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(NoLimitError::Mixer(format!(
                "Failed to get status: {}",
                response.status()
            )));
        }

        let status: MixStatus = response.json().await?;
        Ok(status)
    }

    /// Calculate fee
    pub fn calculate_fee(&self, amount: &str) -> (String, String) {
        let value: f64 = amount.parse().unwrap_or(0.0);
        let fee = value * 0.01;
        let output = value - fee;
        (format!("{:.8}", fee), format!("{:.8}", output))
    }
}

