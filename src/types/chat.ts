/**
 * Chat types
 */

export interface ChatMessage {
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp?: number;
}

export interface ChatOptions {
  /** Previous messages for context */
  history?: ChatMessage[];
  
  /** Request timeout in ms */
  timeout?: number;
}

export interface ChatResponse {
  /** AI response text */
  message: string;
  
  /** x402 payment signature */
  paymentSignature?: string;
}

