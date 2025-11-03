/**
 * Wallet Adapter Example - TypeScript
 * Using with React wallet-adapter
 */

import { NoLimitClient } from '@nolimit/solana-sdk';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { useCallback, useState } from 'react';

// Hook for noLimit client
export function useNoLimit() {
  const { publicKey, signTransaction, signAllTransactions, signMessage } = useWallet();
  const { connection } = useConnection();
  const [client, setClient] = useState<NoLimitClient | null>(null);

  // Initialize client when wallet connects
  const initClient = useCallback(() => {
    if (!publicKey || !signTransaction) return null;

    const walletAdapter = {
      publicKey,
      connected: true,
      signTransaction,
      signAllTransactions: signAllTransactions!,
      signMessage,
    };

    const newClient = NoLimitClient.fromWallet(walletAdapter, {
      connection,
      debug: true,
    });

    setClient(newClient);
    return newClient;
  }, [publicKey, signTransaction, signAllTransactions, signMessage, connection]);

  return { client, initClient };
}

// Example React component
export function ChatComponent() {
  const { client, initClient } = useNoLimit();
  const [message, setMessage] = useState('');
  const [response, setResponse] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSend = async () => {
    let activeClient = client;
    if (!activeClient) {
      activeClient = initClient();
    }
    if (!activeClient) {
      alert('Please connect wallet');
      return;
    }

    setLoading(true);
    try {
      const result = await activeClient.chat.send(message);
      setResponse(result.message);
    } catch (err) {
      console.error(err);
      setResponse('Error: ' + (err as Error).message);
    } finally {
      setLoading(false);
    }
  };

  return {
    message,
    setMessage,
    response,
    loading,
    handleSend,
  };
}

// hook
