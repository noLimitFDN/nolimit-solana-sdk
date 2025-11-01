/**
 * Basic Usage - TypeScript
 * noLimit Solana SDK
 */

import { NoLimitClient } from '@nolimit/solana-sdk';
import { Connection, Keypair } from '@solana/web3.js';
import bs58 from 'bs58';

async function main() {
  // From private key (server-side)
  const privateKey = process.env.SOLANA_PRIVATE_KEY!;
  const keypair = Keypair.fromSecretKey(bs58.decode(privateKey));
  
  const connection = new Connection(
    process.env.RPC_URL || 'https://api.mainnet-beta.solana.com',
    'confirmed'
  );

  const client = NoLimitClient.fromKeypair(keypair, {
    connection,
    debug: true,
  });

  console.log('Wallet:', client.getPublicKey().toBase58());

  // Chat with uncensored AI
  console.log('\n--- Chat ---');
  const chatResponse = await client.chat.send('Explain proof of history');
  console.log('AI:', chatResponse.message);
  console.log('Payment:', chatResponse.paymentSignature);

  // Get swap quote
  console.log('\n--- Swap Quote ---');
  const quote = await client.swap.quote({
    from: 'SOL',
    to: 'USDC',
    amount: '0.1',
  });
  console.log('Output:', quote.outAmount, 'USDC');
  console.log('Price impact:', quote.priceImpactPct, '%');

  // Execute swap
  console.log('\n--- Execute Swap ---');
  const swapResult = await client.swap.execute({
    from: 'SOL',
    to: 'USDC',
    amount: '0.1',
    slippage: 100, // 1%
  });
  console.log('Signature:', swapResult.signature);
  console.log('NL Rewards:', swapResult.nlRewards);

  // Create mix
  console.log('\n--- Mixer ---');
  const mix = await client.mixer.create({
    token: 'USDC',
    amount: '10',
    recipient: 'Gk9WSfyp3GKrLBjVMV4yMg5tLVi3Z3gTrhtfS7dZJWAf',
  });
  console.log('Mix ID:', mix.mixId);
  console.log('Deposit to:', mix.depositAddress);
}

main().catch(console.error);

