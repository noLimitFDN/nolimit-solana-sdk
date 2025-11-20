//! Swap Example - Rust
//! noLimit Solana SDK via Jupiter

use anyhow::Result;
use nolimit_solana::{NoLimitClient, SwapParams};
use solana_sdk::signer::keypair::Keypair;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let private_key = env::var("SOLANA_PRIVATE_KEY")?;
    let keypair_bytes = bs58::decode(&private_key).into_vec()?;
    let keypair = Keypair::from_bytes(&keypair_bytes)?;
    
    let client = NoLimitClient::new(keypair, None)?;
    
    println!("Wallet: {}", client.pubkey());
    
    // Get quote
    println!("\n--- Quote ---");
    let quote = client
        .swap()
        .quote(SwapParams {
            from: "SOL".to_string(),
            to: "USDC".to_string(),
            amount: "0.1".to_string(),
            slippage_bps: Some(50),
        })
        .await?;
    
    println!("Input: {} SOL", quote.in_amount);
    println!("Output: {} USDC", quote.out_amount);
    println!("Price Impact: {}%", quote.price_impact_pct);
    
    // Execute swap
    println!("\n--- Swap ---");
    let result = client
        .swap()
        .execute(SwapParams {
            from: "SOL".to_string(),
            to: "USDC".to_string(),
            amount: "0.1".to_string(),
            slippage_bps: Some(100),
        })
        .await?;
    
    println!("Signature: {}", result.signature);
    println!("Output: {} USDC", result.out_amount);
    println!("NL Rewards: {} $NL", result.nl_rewards);
    
    Ok(())
}

