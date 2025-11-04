/**
 * Basic Usage - JavaScript
 * noLimit Solana SDK
 */

const { NoLimitClient } = require('@nolimit/solana-sdk');
const { Connection, Keypair } = require('@solana/web3.js');
const bs58 = require('bs58');

async function main() {
  // From environment variable
  const privateKey = process.env.SOLANA_PRIVATE_KEY;
  const keypair = Keypair.fromSecretKey(bs58.decode(privateKey));
  
  const connection = new Connection('https://api.mainnet-beta.solana.com', 'confirmed');

  const client = NoLimitClient.fromKeypair(keypair, { connection });

  console.log('Connected:', client.getPublicKey().toBase58());

  // Chat
  const response = await client.chat.send('What is Solana?');
  console.log('AI:', response.message);

  // Quote
  const quote = await client.swap.quote({
    from: 'SOL',
    to: 'USDC',
    amount: '1',
  });
  console.log('Quote:', quote.outAmount, 'USDC');

  // Mix fee calculation
  const { fee, output } = client.mixer.calculateFee('100');
  console.log(`Mix 100 USDC: fee=${fee}, output=${output}`);
}

main().catch(console.error);

