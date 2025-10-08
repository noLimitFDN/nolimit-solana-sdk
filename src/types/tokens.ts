/**
 * Token types for Solana
 */

export type SupportedToken = 'SOL' | 'USDC' | 'USDT';

export interface TokenConfig {
  mint: string;
  decimals: number;
  symbol: string;
  name: string;
}

export const TOKEN_CONFIGS: Record<SupportedToken, TokenConfig> = {
  SOL: {
    mint: 'So11111111111111111111111111111111111111112',
    decimals: 9,
    symbol: 'SOL',
    name: 'Solana',
  },
  USDC: {
    mint: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v',
    decimals: 6,
    symbol: 'USDC',
    name: 'USD Coin',
  },
  USDT: {
    mint: 'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB',
    decimals: 6,
    symbol: 'USDT',
    name: 'Tether USD',
  },
};

export const NATIVE_SOL = 'So11111111111111111111111111111111111111112';
export const USDC_MINT = 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v';

// mint
