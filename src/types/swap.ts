/**
 * Swap types
 */

import type { SupportedToken } from './tokens';

export interface SwapParams {
  /** Source token */
  from: SupportedToken | string;
  
  /** Destination token */
  to: SupportedToken | string;
  
  /** Amount in human readable format */
  amount: string;
  
  /** Slippage in basis points (default: 50 = 0.5%) */
  slippage?: number;
  
  /** Use versioned transaction */
  useVersionedTx?: boolean;
}

export interface SwapQuote {
  /** Input amount (lamports/smallest unit) */
  inAmount: string;
  
  /** Output amount */
  outAmount: string;
  
  /** Minimum output after slippage */
  otherAmountThreshold: string;
  
  /** Price impact percentage */
  priceImpactPct: number;
  
  /** Routing info */
  routePlan: JupiterRoute[];
  
  /** Quote valid for (seconds) */
  contextSlot: number;
}

export interface SwapResult {
  /** Transaction signature */
  signature: string;
  
  /** Input amount */
  inAmount: string;
  
  /** Output amount */
  outAmount: string;
  
  /** $NL rewards earned */
  nlRewards: string;
  
  /** x402 payment signature */
  paymentSignature?: string;
}

export interface JupiterRoute {
  swapInfo: {
    ammKey: string;
    label: string;
    inputMint: string;
    outputMint: string;
    inAmount: string;
    outAmount: string;
    feeAmount: string;
    feeMint: string;
  };
  percent: number;
}

export interface JupiterSwapResponse {
  swapTransaction: string;
  lastValidBlockHeight: number;
}

