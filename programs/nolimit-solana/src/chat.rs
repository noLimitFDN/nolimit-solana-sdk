//! Chat client for noLimit LLM

use crate::{NoLimitError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use solana_sdk::signer::{keypair::Keypair, Signer};
use std::sync::Arc;

/// Chat options
#[derive(Default, Clone)]
pub struct ChatOptions {
    pub history: Option<Vec<(&'static str, &'static str)>>,
    pub timeout_ms: Option<u64>,
}

/// Chat response
#[derive(Debug, Clone)]
pub struct ChatResponse {
    pub message: String,
    pub payment_signature: Option<String>,
}

#[derive(Serialize)]
struct ChatRequest {
    message: String,
    #[serde(rename = "userAddress")]
    user_address: String,
    #[serde(rename = "conversationHistory", skip_serializing_if = "Option::is_none")]
    conversation_history: Option<Vec<HistoryMessage>>,
}

#[derive(Serialize)]
struct HistoryMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatApiResponse {
    response: String,
}

/// Chat client for noLimit LLM
pub struct ChatClient {
    keypair: Arc<Keypair>,
    server_url: String,
    api_key: Option<String>,
    http: Client,
}

impl ChatClient {
    pub fn new(
        keypair: Arc<Keypair>,
        server_url: String,
        api_key: Option<String>,
    ) -> Self {
        Self {
            keypair,
            server_url,
            api_key,
            http: Client::new(),
        }
    }

    /// Send message to AI
    pub async fn send(&self, message: &str) -> Result<ChatResponse> {
        self.send_with_options(message, ChatOptions::default()).await
    }

    /// Send message with options
    pub async fn send_with_options(
        &self,
        message: &str,
        options: ChatOptions,
    ) -> Result<ChatResponse> {
        let endpoint = if self.api_key.is_some() {
            format!("{}/api/agent", self.server_url)
        } else {
            format!("{}/noLimitLLM/solana", self.server_url)
        };

        let history = options.history.map(|h| {
            h.into_iter()
                .map(|(role, content)| HistoryMessage {
                    role: role.to_string(),
                    content: content.to_string(),
                })
                .collect()
        });

        let request = ChatRequest {
            message: message.to_string(),
            user_address: self.keypair.pubkey().to_string(),
            conversation_history: history,
        };

        let mut req_builder = self.http.post(&endpoint).json(&request);

        if let Some(ref api_key) = self.api_key {
            req_builder = req_builder.header("X-API-Key", api_key);
        }

        let response = req_builder.send().await.map_err(|e| {
            NoLimitError::Network(format!("Request failed: {}", e))
        })?;

        if response.status() == 402 {
            // Handle x402 payment
            return Err(NoLimitError::Payment(
                "Payment required - x402 flow not yet implemented in Rust SDK".to_string(),
            ));
        }

        if !response.status().is_success() {
            return Err(NoLimitError::Network(format!(
                "Request failed with status: {}",
                response.status()
            )));
        }

        let api_response: ChatApiResponse = response.json().await.map_err(|e| {
            NoLimitError::Network(format!("Failed to parse response: {}", e))
        })?;

        Ok(ChatResponse {
            message: api_response.response,
            payment_signature: None,
        })
    }
}

