use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;

/// JSON-RPC Client for Dogecoin Node Communication
/// 
/// Provides methods to interact with a running Dogecoind node.
pub struct DogeRpcClient {
    url: String,
    client: reqwest::blocking::Client,
    auth: Option<(String, String)>,
}

/// JSON-RPC Request structure
#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: &'static str,
    id: u64,
    method: String,
    params: Vec<Value>,
}

/// JSON-RPC Response structure
#[derive(Deserialize, Debug)]
struct RpcResponse {
    result: Option<Value>,
    error: Option<RpcError>,
    #[allow(dead_code)]
    id: u64,
}

/// JSON-RPC Error structure
#[derive(Deserialize, Debug)]
struct RpcError {
    code: i32,
    message: String,
}

/// UTXO Information
#[derive(Debug, Clone)]
pub struct UtxoInfo {
    pub txid: String,
    pub vout: u32,
    pub value: u64, // in satoshis
    pub script_pubkey: String,
    pub confirmations: u64,
}

/// Broadcast Result
#[derive(Debug)]
pub struct BroadcastResult {
    pub txid: String,
}

impl DogeRpcClient {
    /// Create a new RPC client
    /// 
    /// # Arguments
    /// * `url` - RPC endpoint URL (e.g., "http://127.0.0.1:44555")
    /// * `username` - Optional RPC username
    /// * `password` - Optional RPC password
    pub fn new(url: &str, username: Option<&str>, password: Option<&str>) -> Self {
        let auth = match (username, password) {
            (Some(u), Some(p)) => Some((u.to_string(), p.to_string())),
            _ => None,
        };

        Self {
            url: url.to_string(),
            client: reqwest::blocking::Client::new(),
            auth,
        }
    }

    /// Send a JSON-RPC request
    fn call(&self, method: &str, params: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0",
            id: 1,
            method: method.to_string(),
            params,
        };

        let mut req_builder = self.client.post(&self.url).json(&request);

        if let Some((ref user, ref pass)) = self.auth {
            req_builder = req_builder.basic_auth(user, Some(pass));
        }

        let response: RpcResponse = req_builder.send()?.json()?;

        if let Some(error) = response.error {
            return Err(format!("RPC Error {}: {}", error.code, error.message).into());
        }

        response.result.ok_or_else(|| "Empty result from RPC".into())
    }

    /// Fetch UTXO details from a transaction
    /// 
    /// # Arguments
    /// * `txid` - Transaction ID in hex
    /// * `vout` - Output index
    pub fn fetch_utxo(&self, txid: &str, vout: u32) -> Result<UtxoInfo, Box<dyn Error>> {
        // First, get the raw transaction with verbose output
        let tx_result = self.call("getrawtransaction", vec![json!(txid), json!(true)])?;

        let outputs = tx_result
            .get("vout")
            .and_then(|v| v.as_array())
            .ok_or("No vout array in transaction")?;

        let output = outputs
            .get(vout as usize)
            .ok_or_else(|| format!("Output index {} not found", vout))?;

        let value_doge: f64 = output
            .get("value")
            .and_then(|v| v.as_f64())
            .ok_or("No value in output")?;

        // Convert DOGE to satoshis (1 DOGE = 100,000,000 satoshis)
        let value_satoshis = (value_doge * 100_000_000.0) as u64;

        let script_pubkey = output
            .get("scriptPubKey")
            .and_then(|s| s.get("hex"))
            .and_then(|h| h.as_str())
            .ok_or("No scriptPubKey hex")?;

        let confirmations = tx_result
            .get("confirmations")
            .and_then(|c| c.as_u64())
            .unwrap_or(0);

        Ok(UtxoInfo {
            txid: txid.to_string(),
            vout,
            value: value_satoshis,
            script_pubkey: script_pubkey.to_string(),
            confirmations,
        })
    }

    /// Broadcast a signed transaction to the network
    /// 
    /// # Arguments
    /// * `tx_hex` - Signed transaction in hex format
    pub fn broadcast_tx(&self, tx_hex: &str) -> Result<BroadcastResult, Box<dyn Error>> {
        let result = self.call("sendrawtransaction", vec![json!(tx_hex)])?;

        let txid = result
            .as_str()
            .ok_or("Expected string txid from sendrawtransaction")?;

        Ok(BroadcastResult {
            txid: txid.to_string(),
        })
    }

    /// Get blockchain info (useful for testing connection)
    pub fn get_blockchain_info(&self) -> Result<Value, Box<dyn Error>> {
        self.call("getblockchaininfo", vec![])
    }

    /// Get network info
    pub fn get_network_info(&self) -> Result<Value, Box<dyn Error>> {
        self.call("getnetworkinfo", vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_client_creation() {
        let client = DogeRpcClient::new("http://localhost:44555", Some("user"), Some("pass"));
        assert_eq!(client.url, "http://localhost:44555");
        assert!(client.auth.is_some());
    }

    #[test]
    fn test_rpc_client_no_auth() {
        let client = DogeRpcClient::new("http://localhost:44555", None, None);
        assert!(client.auth.is_none());
    }
}
