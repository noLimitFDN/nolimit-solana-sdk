"""
noLimit Solana SDK - Python Client
Unofficial Python wrapper for noLimit services on Solana
"""

import os
import json
import base64
import time
from typing import Optional, List, Dict, Any
from dataclasses import dataclass

import requests
from solders.keypair import Keypair
from solders.pubkey import Pubkey
from solders.transaction import VersionedTransaction
from solana.rpc.api import Client as SolanaClient


@dataclass
class ChatMessage:
    role: str
    content: str


@dataclass
class ChatResponse:
    message: str
    payment_signature: Optional[str] = None


@dataclass
class SwapQuote:
    in_amount: str
    out_amount: str
    price_impact_pct: float


@dataclass
class SwapResult:
    signature: str
    in_amount: str
    out_amount: str
    nl_rewards: str


@dataclass
class MixResult:
    mix_id: str
    deposit_address: str
    deposit_amount: str
    fee: str
    output_amount: str


class NoLimitClient:
    """
    Python client for noLimit on Solana.
    
    Usage:
        client = NoLimitClient(private_key=bytes([...]))
        response = client.chat.send("Hello")
        print(response.message)
    """
    
    DEFAULT_SERVER = "https://x402.nolimit.foundation"
    DEFAULT_RPC = "https://api.mainnet-beta.solana.com"
    
    TOKENS = {
        "SOL": "So11111111111111111111111111111111111111112",
        "USDC": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        "USDT": "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
    }
    
    def __init__(
        self,
        private_key: Optional[bytes] = None,
        api_key: Optional[str] = None,
        server_url: Optional[str] = None,
        rpc_url: Optional[str] = None,
    ):
        self.server_url = server_url or self.DEFAULT_SERVER
        self.api_key = api_key
        self.rpc = SolanaClient(rpc_url or self.DEFAULT_RPC)
        
        if private_key:
            self.keypair = Keypair.from_bytes(private_key)
            self.pubkey = self.keypair.pubkey()
        else:
            self.keypair = None
            self.pubkey = None
            
        self.chat = ChatClient(self)
        self.swap = SwapClient(self)
        self.mixer = MixerClient(self)
    
    def _request(
        self,
        endpoint: str,
        body: Dict[str, Any],
        timeout: int = 60
    ) -> Dict[str, Any]:
        """Make request with x402 payment or API key"""
        url = f"{self.server_url}{endpoint}"
        headers = {"Content-Type": "application/json"}
        
        if self.api_key:
            headers["X-API-Key"] = self.api_key
            response = requests.post(url, json=body, headers=headers, timeout=timeout)
            response.raise_for_status()
            return {"data": response.json()}
        
        # x402 payment flow
        response = requests.post(url, json=body, headers=headers, timeout=timeout)
        
        if response.status_code != 402:
            response.raise_for_status()
            return {"data": response.json()}
        
        # Create payment
        payment_req = response.json()
        accepts = payment_req.get("accepts", [{}])[0]
        
        payment_header = self._create_payment(accepts)
        headers["X-Payment"] = payment_header
        
        response = requests.post(url, json=body, headers=headers, timeout=timeout)
        response.raise_for_status()
        
        return {
            "data": response.json(),
            "payment_signature": response.headers.get("X-Payment-Response")
        }
    
    def _create_payment(self, requirements: Dict[str, Any]) -> str:
        """Create x402 payment header"""
        if not self.keypair:
            raise ValueError("Keypair required for payment")
        
        payload = {
            "version": "1",
            "network": "solana",
            "from": str(self.pubkey),
            "to": requirements.get("payTo"),
            "amount": requirements.get("maxAmountRequired"),
            "asset": requirements.get("asset", {}).get("address"),
            "resource": requirements.get("resource"),
            "timestamp": int(time.time() * 1000)
        }
        
        payload_bytes = json.dumps(payload).encode()
        signature = self.keypair.sign_message(payload_bytes)
        
        full_payload = {**payload, "signature": base64.b64encode(bytes(signature)).decode()}
        return base64.b64encode(json.dumps(full_payload).encode()).decode()


class ChatClient:
    """Chat client for noLimit LLM"""
    
    def __init__(self, client: NoLimitClient):
        self._client = client
    
    def send(
        self,
        message: str,
        history: Optional[List[ChatMessage]] = None,
    ) -> ChatResponse:
        """Send message to AI"""
        body = {
            "message": message,
            "userAddress": str(self._client.pubkey) if self._client.pubkey else "anonymous",
        }
        
        if history:
            body["conversationHistory"] = [
                {"role": m.role, "content": m.content} for m in history
            ]
        
        endpoint = "/api/agent" if self._client.api_key else "/noLimitLLM/solana"
        result = self._client._request(endpoint, body, 60)
        
        return ChatResponse(
            message=result["data"]["response"],
            payment_signature=result.get("payment_signature")
        )


class SwapClient:
    """Swap client via Jupiter"""
    
    JUPITER_API = "https://lite-api.jup.ag/swap/v1"
    
    def __init__(self, client: NoLimitClient):
        self._client = client
    
    def quote(
        self,
        from_token: str,
        to_token: str,
        amount: str,
        slippage: int = 50
    ) -> SwapQuote:
        """Get swap quote"""
        input_mint = NoLimitClient.TOKENS.get(from_token.upper(), from_token)
        output_mint = NoLimitClient.TOKENS.get(to_token.upper(), to_token)
        
        decimals = 9 if from_token.upper() == "SOL" else 6
        amount_raw = str(int(float(amount) * (10 ** decimals)))
        
        url = f"{self.JUPITER_API}/quote?inputMint={input_mint}&outputMint={output_mint}&amount={amount_raw}&slippageBps={slippage}"
        response = requests.get(url)
        response.raise_for_status()
        data = response.json()
        
        return SwapQuote(
            in_amount=data["inAmount"],
            out_amount=data["outAmount"],
            price_impact_pct=float(data.get("priceImpactPct", 0))
        )
    
    def execute(
        self,
        from_token: str,
        to_token: str,
        amount: str,
        slippage: int = 50
    ) -> SwapResult:
        """Execute swap"""
        input_mint = NoLimitClient.TOKENS.get(from_token.upper(), from_token)
        output_mint = NoLimitClient.TOKENS.get(to_token.upper(), to_token)
        
        decimals = 9 if from_token.upper() == "SOL" else 6
        amount_raw = str(int(float(amount) * (10 ** decimals)))
        
        body = {
            "chain": "solana",
            "fromToken": input_mint,
            "toToken": output_mint,
            "amount": amount_raw,
            "userAddress": str(self._client.pubkey),
            "slippage": slippage,
        }
        
        result = self._client._request("/noLimitSwap/solana", body, 120)
        data = result["data"]
        
        # Sign and send transaction
        tx_bytes = base64.b64decode(data["tx"])
        tx = VersionedTransaction.from_bytes(tx_bytes)
        tx.sign([self._client.keypair])
        
        sig = self._client.rpc.send_transaction(tx).value
        
        return SwapResult(
            signature=str(sig),
            in_amount=amount_raw,
            out_amount=data["quote"]["toAmount"],
            nl_rewards=data.get("nlEarned", "0")
        )


class MixerClient:
    """Mixer client"""
    
    def __init__(self, client: NoLimitClient):
        self._client = client
    
    def create(
        self,
        token: str,
        amount: str,
        recipient: str,
        delay: int = 0
    ) -> MixResult:
        """Create mix request"""
        body = {
            "token": token,
            "amount": amount,
            "recipientAddress": recipient,
            "userAddress": str(self._client.pubkey),
            "delayMinutes": delay,
        }
        
        result = self._client._request("/noLimitMixer/solana", body, 30)
        data = result["data"]
        
        return MixResult(
            mix_id=data["mixId"],
            deposit_address=data["depositAddress"],
            deposit_amount=data["depositAmount"],
            fee=data["fee"],
            output_amount=data["outputAmount"]
        )
    
    def get_status(self, mix_id: str) -> Dict[str, Any]:
        """Get mix status"""
        url = f"{self._client.server_url}/mixer/status/{mix_id}"
        response = requests.get(url)
        response.raise_for_status()
        return response.json()


if __name__ == "__main__":
    # Example usage
    import base58
    
    private_key = base58.b58decode(os.environ.get("SOLANA_PRIVATE_KEY", ""))
    client = NoLimitClient(private_key=private_key)
    
    response = client.chat.send("What is Solana?")
    print(f"AI: {response.message}")

#   s w a p  
 