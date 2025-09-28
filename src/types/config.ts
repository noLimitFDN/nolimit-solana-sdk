/**
 * Configuration types
 */

import type { Connection, Keypair, PublicKey, Transaction, VersionedTransaction } from '@solana/web3.js';

export interface NoLimitConfig {
  /** Solana wallet adapter */
  wallet?: WalletAdapter;
  
  /** Keypair for signing */
  keypair?: Keypair;
  
  /** Solana connection */
  connection?: Connection;
  
  /** RPC URL (if no connection provided) */
  rpcUrl?: string;
  
  /** Enterprise API key */
  apiKey?: string;
  
  /** Custom server URL */
  serverUrl?: string;
  
  /** Enable debug logs */
  debug?: boolean;
  
  /** Commitment level */
  commitment?: 'processed' | 'confirmed' | 'finalized';
}

export interface WalletAdapter {
  publicKey: PublicKey | null;
  connected: boolean;
  signTransaction<T extends Transaction | VersionedTransaction>(tx: T): Promise<T>;
  signAllTransactions<T extends Transaction | VersionedTransaction>(txs: T[]): Promise<T[]>;
  signMessage?(message: Uint8Array): Promise<Uint8Array>;
}

export interface SignedTransaction {
  signature: string;
  transaction: Transaction | VersionedTransaction;
}

// keypair
