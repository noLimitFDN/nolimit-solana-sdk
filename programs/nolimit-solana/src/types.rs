//! Common types

use serde::{Deserialize, Serialize};

/// Supported tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Sol,
    Usdc,
    Usdt,
}

impl Token {
    pub fn mint(&self) -> &'static str {
        match self {
            Token::Sol => "So11111111111111111111111111111111111111112",
            Token::Usdc => "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            Token::Usdt => "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
        }
    }

    pub fn decimals(&self) -> u8 {
        match self {
            Token::Sol => 9,
            Token::Usdc | Token::Usdt => 6,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Token::Sol => "SOL",
            Token::Usdc => "USDC",
            Token::Usdt => "USDT",
        }
    }
}

impl std::str::FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SOL" => Ok(Token::Sol),
            "USDC" => Ok(Token::Usdc),
            "USDT" => Ok(Token::Usdt),
            _ => Err(format!("Unknown token: {}", s)),
        }
    }
}

/// x402 payment header
#[derive(Serialize, Deserialize)]
pub struct X402Payload {
    pub version: String,
    pub network: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub asset: String,
    pub resource: String,
    pub timestamp: u64,
    pub signature: Option<String>,
}

