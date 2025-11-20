//! Chat Example - Rust
//! noLimit Solana SDK

use anyhow::Result;
use nolimit_solana::{NoLimitClient, ChatOptions};
use solana_sdk::signer::keypair::Keypair;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load keypair from environment
    let private_key = env::var("SOLANA_PRIVATE_KEY")
        .expect("SOLANA_PRIVATE_KEY not set");
    
    let keypair_bytes = bs58::decode(&private_key).into_vec()?;
    let keypair = Keypair::from_bytes(&keypair_bytes)?;
    
    // Create client
    let client = NoLimitClient::new(keypair, None)?;
    
    println!("Wallet: {}", client.pubkey());
    
    // Simple chat
    println!("\n--- Simple Chat ---");
    let response = client
        .chat()
        .send("Explain proof of history in Solana")
        .await?;
    
    println!("AI: {}", response.message);
    if let Some(sig) = response.payment_signature {
        println!("Payment: {}", sig);
    }
    
    // Chat with history
    println!("\n--- Chat with Context ---");
    let history = vec![
        ("user", "What is Solana?"),
        ("assistant", "Solana is a high-performance blockchain..."),
    ];
    
    let response = client
        .chat()
        .send_with_options(
            "How fast is it?",
            ChatOptions {
                history: Some(history),
                ..Default::default()
            },
        )
        .await?;
    
    println!("AI: {}", response.message);
    
    Ok(())
}

