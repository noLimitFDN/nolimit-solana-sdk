# @nolimit/solana-sdk

Official Solana SDK for noLimit Foundation - Privacy-first AI, Swap, and Mixer

[![npm](https://img.shields.io/npm/v/@nolimit/solana-sdk)](https://www.npmjs.com/package/@nolimit/solana-sdk)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Integrate noLimit's privacy utilities into your Solana dApp:

- **noLimit LLM** - Uncensored AI with zero data retention
- **noLimit Swap** - Privacy-enhanced DEX via Jupiter
- **noLimit Mixer** - Break on-chain transaction links

All services use [x402-solana](https://github.com/PayAINetwork/x402-solana) for trustless micropayments.

## Installation

```bash
npm install @nolimit/solana-sdk @solana/web3.js @solana/spl-token
```

## Quick Start

### TypeScript

```typescript
import { NoLimitClient } from '@nolimit/solana-sdk';
import { Connection, Keypair } from '@solana/web3.js';

// From wallet adapter
const client = new NoLimitClient({
  wallet: walletAdapter,
  connection: new Connection('https://api.mainnet-beta.solana.com'),
});

// Or from keypair
const client = NoLimitClient.fromKeypair(keypair);

// Chat - $0.05/message
const response = await client.chat.send('Explain proof of history');
console.log(response.message);

// Swap via Jupiter - $0.10/swap
const swap = await client.swap.execute({
  from: 'SOL',
  to: 'USDC',
  amount: '1.0',
});

// Mixer - $0.075 + 1% fee
const mix = await client.mixer.create({
  token: 'USDC',
  amount: '500',
  recipient: 'Gk9W...',
});
```

### Python

```python
from nolimit_solana import NoLimitClient

client = NoLimitClient(private_key=bytes([...]))
response = client.chat.send("How do SPL tokens work?")
```

### Rust

```rust
use nolimit_solana::NoLimitClient;

let client = NoLimitClient::new(keypair);
let response = client.chat().send("Explain Solana").await?;
```

## Token Addresses (Solana Mainnet)

| Token | Mint Address |
|-------|-------------|
| SOL | Native |
| USDC | `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v` |
| USDT | `Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB` |

## Pricing

| Service | Cost |
|---------|------|
| Chat | $0.05 USDC / message |
| Swap | $0.10 USDC / transaction |
| Mixer | $0.075 USDC + 1% |

## API Reference

### Chat

```typescript
await client.chat.send(message, { history });
```

### Swap (Jupiter)

```typescript
await client.swap.execute({
  from: 'SOL',
  to: 'USDC',
  amount: '1.0',
  slippage: 50, // basis points
});

const quote = await client.swap.quote({ from, to, amount });
```

### Mixer

```typescript
const mix = await client.mixer.create({
  token: 'USDC',
  amount: '100',
  recipient: 'pubkey...',
  delay: 5,
});

const status = await client.mixer.getStatus(mix.mixId);
```

## Links

- [Documentation](https://docs.nolimit.foundation/solana)
- [Integration Guide](https://nolimit.foundation/integration)
- [Discord](https://discord.gg/nolimit)
- [Twitter](https://twitter.com/noLimitFDN)

## License

MIT - noLimit Foundation
<!-- v1 -->
