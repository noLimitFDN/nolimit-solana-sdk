//! Swap client via Jupiter

use crate::{NoLimitError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::Signature,
    signer::{keypair::Keypair, Signer},
    transaction::VersionedTransaction,
};
use std::sync::Arc;

/// Swap parameters
#[derive(Clone)]
pub struct SwapParams {
    pub from: String,
    pub to: String,
    pub amount: String,
    pub slippage_bps: Option<u16>,
}

/// Swap quote
#[derive(Debug, Clone)]
pub struct SwapQuote {
    pub in_amount: String,
    pub out_amount: String,
    pub price_impact_pct: f64,
}

/// Swap result
#[derive(Debug, Clone)]
pub struct SwapResult {
    pub signature: String,
    pub in_amount: String,
    pub out_amount: String,
    pub nl_rewards: String,
}

#[derive(Deserialize)]
struct JupiterQuoteResponse {
    #[serde(rename = "inAmount")]
    in_amount: String,
    #[serde(rename = "outAmount")]
    out_amount: String,
    #[serde(rename = "priceImpactPct")]
    price_impact_pct: Option<String>,
}

/// Swap client
pub struct SwapClient {
    keypair: Arc<Keypair>,
    rpc: Arc<RpcClient>,
    server_url: String,
    http: Client,
}

impl SwapClient {
    const JUPITER_API: &'static str = "https://lite-api.jup.ag/swap/v1";

    const TOKENS: &'static [(&'static str, &'static str)] = &[
        ("SOL", "So11111111111111111111111111111111111111112"),
        ("USDC", "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
        ("USDT", "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"),
    ];

    pub fn new(
        keypair: Arc<Keypair>,
        rpc: Arc<RpcClient>,
        server_url: String,
    ) -> Self {
        Self {
            keypair,
            rpc,
            server_url,
            http: Client::new(),
        }
    }

    /// Get swap quote from Jupiter
    pub async fn quote(&self, params: SwapParams) -> Result<SwapQuote> {
        let input_mint = self.resolve_mint(&params.from);
        let output_mint = self.resolve_mint(&params.to);
        let amount = self.parse_amount(&params.amount, &params.from);
        let slippage = params.slippage_bps.unwrap_or(50);

        let url = format!(
            "{}/quote?inputMint={}&outputMint={}&amount={}&slippageBps={}",
            Self::JUPITER_API, input_mint, output_mint, amount, slippage
        );

        let response: JupiterQuoteResponse = self
            .http
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        Ok(SwapQuote {
            in_amount: response.in_amount,
            out_amount: response.out_amount,
            price_impact_pct: response
                .price_impact_pct
                .map(|s| s.parse().unwrap_or(0.0))
                .unwrap_or(0.0),
        })
    }

    /// Execute swap
    pub async fn execute(&self, params: SwapParams) -> Result<SwapResult> {
        // For full implementation, this would:
        // 1. Call noLimit x402 endpoint to get swap tx
        // 2. Sign and send transaction
        // 3. Return result
        
        Err(NoLimitError::Unknown(
            "Full swap implementation requires x402 payment flow".to_string(),
        ))
    }

    fn resolve_mint(&self, token: &str) -> String {
        Self::TOKENS
            .iter()
            .find(|(sym, _)| sym.eq_ignore_ascii_case(token))
            .map(|(_, mint)| mint.to_string())
            .unwrap_or_else(|| token.to_string())
    }

    fn parse_amount(&self, amount: &str, token: &str) -> String {
        let decimals = if token.eq_ignore_ascii_case("SOL") { 9 } else { 6 };
        let value: f64 = amount.parse().unwrap_or(0.0);
        let raw = (value * 10f64.powi(decimals)) as u64;
        raw.to_string()
    }
}

