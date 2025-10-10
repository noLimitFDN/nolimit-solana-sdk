/**
 * Solana SDK Error Classes
 */

export class NoLimitError extends Error {
  readonly code: string;
  readonly details?: Record<string, unknown>;

  constructor(message: string, code: string, details?: Record<string, unknown>) {
    super(message);
    this.name = 'NoLimitError';
    this.code = code;
    this.details = details;
    Object.setPrototypeOf(this, NoLimitError.prototype);
  }
}

export class PaymentError extends NoLimitError {
  readonly required?: string;
  readonly payTo?: string;

  constructor(message: string, details?: { required?: string; payTo?: string }) {
    super(message, 'PAYMENT_REQUIRED', details);
    this.name = 'PaymentError';
    this.required = details?.required;
    this.payTo = details?.payTo;
    Object.setPrototypeOf(this, PaymentError.prototype);
  }
}

export class NetworkError extends NoLimitError {
  readonly statusCode?: number;

  constructor(message: string, details?: { statusCode?: number; endpoint?: string }) {
    super(message, 'NETWORK_ERROR', details);
    this.name = 'NetworkError';
    this.statusCode = details?.statusCode;
    Object.setPrototypeOf(this, NetworkError.prototype);
  }
}

export class ValidationError extends NoLimitError {
  readonly field?: string;

  constructor(message: string, field?: string) {
    super(message, 'VALIDATION_ERROR', { field });
    this.name = 'ValidationError';
    this.field = field;
    Object.setPrototypeOf(this, ValidationError.prototype);
  }
}

export class WalletError extends NoLimitError {
  constructor(message: string) {
    super(message, 'WALLET_ERROR');
    this.name = 'WalletError';
    Object.setPrototypeOf(this, WalletError.prototype);
  }
}

export class TransactionError extends NoLimitError {
  readonly signature?: string;

  constructor(message: string, signature?: string) {
    super(message, 'TRANSACTION_ERROR', { signature });
    this.name = 'TransactionError';
    this.signature = signature;
    Object.setPrototypeOf(this, TransactionError.prototype);
  }
}

export class MixerError extends NoLimitError {
  readonly mixId?: string;

  constructor(message: string, mixId?: string) {
    super(message, 'MIXER_ERROR', { mixId });
    this.name = 'MixerError';
    this.mixId = mixId;
    Object.setPrototypeOf(this, MixerError.prototype);
  }
}

