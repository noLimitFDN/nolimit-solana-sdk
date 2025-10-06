/**
 * Mixer types
 */

import type { SupportedToken } from './tokens';

export type MixStatusType = 
  | 'pending_deposit'
  | 'deposited'
  | 'mixing'
  | 'completed'
  | 'failed'
  | 'expired';

export interface MixParams {
  /** Token to mix */
  token: SupportedToken;
  
  /** Amount in human readable format */
  amount: string;
  
  /** Recipient public key */
  recipient: string;
  
  /** Delay in minutes (0 = instant) */
  delay?: number;
}

export interface MixResult {
  /** Unique mix identifier */
  mixId: string;
  
  /** Deposit address (public key) */
  depositAddress: string;
  
  /** Amount to deposit */
  depositAmount: string;
  
  /** Fee deducted (1%) */
  fee: string;
  
  /** Amount recipient receives */
  outputAmount: string;
  
  /** Deposit deadline */
  expiresAt: string;
  
  /** x402 payment signature */
  paymentSignature?: string;
}

export interface MixStatus {
  mixId: string;
  status: MixStatusType;
  progress: number;
  currentHop: number;
  totalHops: number;
  completedAt?: string;
  outputSignature?: string;
  error?: string;
}

// hop
