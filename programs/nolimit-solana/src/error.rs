//! Error types

use thiserror::Error;

/// Result type alias
pub type Result<T> = std::result::Result<T, NoLimitError>;

/// SDK errors
#[derive(Error, Debug)]
pub enum NoLimitError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Payment required: {0}")]
    Payment(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Wallet error: {0}")]
    Wallet(String),

    #[error("Mixer error: {0}")]
    Mixer(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<reqwest::Error> for NoLimitError {
    fn from(err: reqwest::Error) -> Self {
        NoLimitError::Network(err.to_string())
    }
}

impl From<solana_sdk::signer::SignerError> for NoLimitError {
    fn from(err: solana_sdk::signer::SignerError) -> Self {
        NoLimitError::Wallet(err.to_string())
    }
}

impl From<solana_client::client_error::ClientError> for NoLimitError {
    fn from(err: solana_client::client_error::ClientError) -> Self {
        NoLimitError::Transaction(err.to_string())
    }
}

