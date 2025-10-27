/**
 * Utility functions
 */

import { PublicKey } from '@solana/web3.js';

/**
 * Parse amount to lamports/smallest unit
 */
export function parseAmount(amount: string, decimals: number): string {
  const [whole, fraction = ''] = amount.split('.');
  const padded = fraction.padEnd(decimals, '0').slice(0, decimals);
  return BigInt(whole + padded).toString();
}

/**
 * Format lamports to human readable
 */
export function formatAmount(lamports: string, decimals: number): string {
  const value = lamports.padStart(decimals + 1, '0');
  const whole = value.slice(0, -decimals) || '0';
  const fraction = value.slice(-decimals).replace(/0+$/, '');
  return fraction ? `${whole}.${fraction}` : whole;
}

/**
 * Validate Solana public key
 */
export function isValidPublicKey(address: string): boolean {
  try {
    new PublicKey(address);
    return true;
  } catch {
    return false;
  }
}

/**
 * Truncate public key for display
 */
export function truncateKey(key: string, chars = 4): string {
  if (!key) return '';
  return `${key.slice(0, chars)}...${key.slice(-chars)}`;
}

/**
 * Sleep utility
 */
export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * Retry with backoff
 */
export async function retry<T>(
  fn: () => Promise<T>,
  options: { retries?: number; delay?: number } = {}
): Promise<T> {
  const { retries = 3, delay = 1000 } = options;
  
  let lastError: Error | undefined;
  
  for (let i = 0; i < retries; i++) {
    try {
      return await fn();
    } catch (err) {
      lastError = err as Error;
      if (i < retries - 1) {
        await sleep(delay * Math.pow(2, i));
      }
    }
  }
  
  throw lastError;
}

