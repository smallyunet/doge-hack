use serde::Deserialize;
use std::error::Error;

use crate::network::Network;

#[derive(Debug, Clone, Copy)]
pub enum ExplorerNetwork {
    Doge,
    DogeTest,
}

impl ExplorerNetwork {
    pub fn from_network(network: Network) -> Self {
        match network {
            Network::Testnet => ExplorerNetwork::DogeTest,
            Network::Mainnet => ExplorerNetwork::Doge,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ExplorerNetwork::Doge => "DOGE",
            ExplorerNetwork::DogeTest => "DOGETEST",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExplorerUtxo {
    pub txid: String,
    pub vout: u32,
    pub value_satoshis: u64,
    pub script_hex: String,
    pub confirmations: u64,
}

/// Chain.so public API client.
///
/// Docs (high-level): https://chain.so/api
pub struct ChainSoClient {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl ChainSoClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://chain.so/api/v2".to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get_tx_unspent(&self, address: &str, network: Network) -> Result<Vec<ExplorerUtxo>, Box<dyn Error>> {
        let net = ExplorerNetwork::from_network(network).as_str();
        let url = format!("{}/get_tx_unspent/{}/{}", self.base_url, net, address);

        let resp: ChainSoEnvelope<ChainSoTxUnspentData> = self.client.get(url).send()?.json()?;
        if resp.status != "success" {
            return Err(format!("chain.so status: {}", resp.status).into());
        }

        let mut utxos = Vec::new();
        for u in resp.data.txs {
            let value_satoshis = (u.value.parse::<f64>()? * 100_000_000.0) as u64;
            let confirmations = u.confirmations.unwrap_or(0);
            utxos.push(ExplorerUtxo {
                txid: u.txid,
                vout: u.output_no,
                value_satoshis,
                script_hex: u.script_hex,
                confirmations,
            });
        }

        Ok(utxos)
    }

    pub fn send_tx(&self, tx_hex: &str, network: Network) -> Result<String, Box<dyn Error>> {
        let net = ExplorerNetwork::from_network(network).as_str();
        let url = format!("{}/send_tx/{}/", self.base_url, net);

        let req = ChainSoSendTxRequest { tx_hex };
        let resp: ChainSoEnvelope<ChainSoSendTxData> = self.client.post(url).json(&req).send()?.json()?;
        if resp.status != "success" {
            return Err(format!("chain.so status: {}", resp.status).into());
        }

        Ok(resp.data.txid)
    }
}

/// SoChain v3 client (requires API key).
///
/// This is used for fetching prevout details by (txid, vout) when constructing spendable transactions.
pub struct SoChainV3Client {
    base_url: String,
    api_key: String,
    client: reqwest::blocking::Client,
}

impl SoChainV3Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            base_url: "https://chain.so/api/v3".to_string(),
            api_key: api_key.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn fetch_output(&self, txid: &str, vout: u32, network: Network) -> Result<ExplorerUtxo, Box<dyn Error>> {
        let net = ExplorerNetwork::from_network(network).as_str();
        let url = format!("{}/transaction/{}/{}", self.base_url, net, txid);

        let resp: SoChainV3Envelope<SoChainV3Transaction> = self
            .client
            .get(url)
            .header("API-KEY", &self.api_key)
            .send()?
            .json()?;

        if resp.status != "success" {
            return Err(format!("chain.so v3 status: {}", resp.status).into());
        }

        let output = resp
            .data
            .outputs
            .iter()
            .find(|o| o.index == vout)
            .ok_or_else(|| format!("output index {} not found", vout))?;

        let value_satoshis = (output.value.parse::<f64>()? * 100_000_000.0) as u64;
        let confirmations = resp.data.confirmations.unwrap_or(0);
        let script_hex = output
            .script
            .as_ref()
            .and_then(|s| s.hex.as_deref())
            .unwrap_or("")
            .to_string();

        if script_hex.is_empty() {
            return Err("missing script hex in chain.so v3 response".into());
        }

        Ok(ExplorerUtxo {
            txid: txid.to_string(),
            vout,
            value_satoshis,
            script_hex,
            confirmations,
        })
    }
}

#[derive(Debug, Deserialize)]
struct SoChainV3Envelope<T> {
    status: String,
    data: T,
}

#[derive(Debug, Deserialize)]
struct SoChainV3Transaction {
    #[serde(default)]
    confirmations: Option<u64>,
    outputs: Vec<SoChainV3Output>,
}

#[derive(Debug, Deserialize)]
struct SoChainV3Output {
    index: u32,
    value: String,
    #[serde(default)]
    script: Option<SoChainV3Script>,
}

#[derive(Debug, Deserialize)]
struct SoChainV3Script {
    #[serde(default)]
    hex: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChainSoEnvelope<T> {
    status: String,
    data: T,
}

#[derive(Debug, Deserialize)]
struct ChainSoTxUnspentData {
    txs: Vec<ChainSoUnspentTx>,
}

#[derive(Debug, Deserialize)]
struct ChainSoUnspentTx {
    txid: String,
    output_no: u32,
    value: String,
    script_hex: String,
    #[serde(default)]
    confirmations: Option<u64>,
}

#[derive(Debug, serde::Serialize)]
struct ChainSoSendTxRequest<'a> {
    tx_hex: &'a str,
}

#[derive(Debug, Deserialize)]
struct ChainSoSendTxData {
    txid: String,
}
