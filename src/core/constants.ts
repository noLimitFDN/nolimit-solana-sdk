/**
 * Solana SDK Constants
 */

export const DEFAULT_SERVER = 'https://x402.nolimit.foundation';
export const DEFAULT_RPC = 'https://api.mainnet-beta.solana.com';

export const ENDPOINTS = {
  CHAT: '/noLimitLLM/solana',
  CHAT_API: '/api/agent',
  SWAP: '/noLimitSwap/solana',
  MIXER: '/noLimitMixer/solana',
  MIX_STATUS: '/mixer/status',
  MIX_CONFIRM: '/mixer/confirm-deposit',
} as const;

export const TOKENS = {
  SOL: 'So11111111111111111111111111111111111111112',
  USDC: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v',
  USDT: 'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB',
} as const;

export const DECIMALS: Record<string, number> = {
  SOL: 9,
  USDC: 6,
  USDT: 6,
};

export const PRICING = {
  CHAT: 0.05,
  SWAP: 0.10,
  MIXER_BASE: 0.075,
  MIXER_PERCENT: 1,
} as const;

export const TIMEOUTS = {
  CHAT: 60_000,
  SWAP: 120_000,
  MIXER: 30_000,
  DEFAULT: 30_000,
} as const;

// Jupiter API
export const JUPITER_API = 'https://lite-api.jup.ag/swap/v1';

// x402 Facilitator
export const FACILITATOR_URL = 'https://facilitator.payai.network';

// jupiter
