use clap::{Parser, Subcommand, ValueEnum};
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::consensus::encode::serialize_hex;
use rand::Rng;

use doge_hack::address::DogeAddress;
use doge_hack::network::Network;
use doge_hack::transaction::TransactionBuilder;
use doge_hack::rpc::DogeRpcClient;
use doge_hack::script;
use doge_hack::explorer::{ChainSoClient, SoChainV3Client};

/// Doge-Hack: Dogecoin Transaction Constructor
/// 
/// Construct valid Dogecoin transactions using Bitcoin primitives.
/// Proves that Dogecoin is essentially "Bitcoin in a yellow vest".
#[derive(Parser)]
#[command(name = "doge-hack")]
#[command(version = "0.0.3")]
#[command(about = "Dogecoin Transaction Constructor using Bitcoin tools", long_about = None)]
struct Cli {
    /// Network to use (testnet or mainnet)
    #[arg(short, long, default_value = "testnet")]
    network: NetworkArg,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
enum NetworkArg {
    Testnet,
    Mainnet,
}

impl From<NetworkArg> for Network {
    fn from(arg: NetworkArg) -> Self {
        match arg {
            NetworkArg::Testnet => Network::Testnet,
            NetworkArg::Mainnet => Network::Mainnet,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new keypair and address
    GenKey,
    
    /// Derive address from a secret key
    Address {
        /// Secret key in hex format (64 characters)
        #[arg(short, long)]
        secret_key: String,
    },
    
    /// Construct and sign a transaction
    Sign {
        /// Transaction ID of the UTXO to spend
        #[arg(long)]
        txid: String,
        
        /// Output index of the UTXO
        #[arg(long)]
        vout: u32,
        
        /// Secret key in hex format
        #[arg(short, long)]
        secret_key: String,
        
        /// Destination address
        #[arg(short, long)]
        to: String,
        
        /// Amount in satoshis
        #[arg(short, long)]
        amount: u64,

        /// Fee in satoshis (default: 0.01 DOGE)
        #[arg(long, default_value_t = 1_000_000)]
        fee: u64,

        /// Change address (defaults to derived from-address)
        #[arg(long)]
        change_address: Option<String>,

        /// Prevout amount in satoshis (if not using RPC/public API)
        #[arg(long)]
        prev_value: Option<u64>,

        /// Prevout scriptPubKey hex (if not using RPC/public API)
        #[arg(long)]
        prev_script_hex: Option<String>,

        /// RPC URL for auto fetching prevout (recommended)
        #[arg(long)]
        rpc_url: Option<String>,

        /// RPC username (optional)
        #[arg(long)]
        rpc_user: Option<String>,

        /// RPC password (optional)
        #[arg(long)]
        rpc_pass: Option<String>,

        /// SoChain API key for public auto fetching prevout (chain.so v3)
        /// If omitted, will also try env var CHAIN_SO_API_KEY.
        #[arg(long)]
        api_key: Option<String>,
    },

    /// Create a P2SH multisig address from pubkeys
    P2shMultisig {
        /// Required signatures (m)
        #[arg(short = 'm', long)]
        required: u8,

        /// Compressed pubkeys in hex (repeatable)
        #[arg(short = 'k', long = "pubkey")]
        pubkeys: Vec<String>,
    },

    /// Fetch UTXOs for an address using a public explorer API (Chain.so)
    FetchUtxosAddress {
        /// Address to query
        #[arg(short, long)]
        address: String,
    },

    /// Broadcast a signed transaction using a public explorer API (Chain.so)
    BroadcastPublic {
        /// Signed transaction hex
        #[arg(short, long)]
        tx_hex: String,
    },
    
    /// Broadcast a signed transaction (requires RPC)
    Broadcast {
        /// Signed transaction hex
        #[arg(short, long)]
        tx_hex: String,
        
        /// RPC URL (e.g., http://127.0.0.1:44555)
        #[arg(long)]
        rpc_url: String,
        
        /// RPC username (optional)
        #[arg(long)]
        rpc_user: Option<String>,
        
        /// RPC password (optional)
        #[arg(long)]
        rpc_pass: Option<String>,
    },
    
    /// Fetch UTXO information (requires RPC)
    FetchUtxo {
        /// Transaction ID
        #[arg(long)]
        txid: String,
        
        /// Output index
        #[arg(long)]
        vout: u32,
        
        /// RPC URL (e.g., http://127.0.0.1:44555)
        #[arg(long)]
        rpc_url: String,
        
        /// RPC username (optional)
        #[arg(long)]
        rpc_user: Option<String>,
        
        /// RPC password (optional)
        #[arg(long)]
        rpc_pass: Option<String>,
    },
    
    /// Run demo mode (original behavior)
    Demo,
}

fn main() {
    let cli = Cli::parse();
    let network: Network = cli.network.into();

    match cli.command {
        Commands::GenKey => cmd_gen_key(network),
        Commands::Address { secret_key } => cmd_address(&secret_key, network),
        Commands::Sign {
            txid,
            vout,
            secret_key,
            to,
            amount,
            fee,
            change_address,
            prev_value,
            prev_script_hex,
            rpc_url,
            rpc_user,
            rpc_pass,
            api_key,
        } => cmd_sign(
            &txid,
            vout,
            &secret_key,
            &to,
            amount,
            fee,
            change_address.as_deref(),
            prev_value,
            prev_script_hex.as_deref(),
            rpc_url.as_deref(),
            rpc_user.as_deref(),
            rpc_pass.as_deref(),
            api_key.as_deref(),
            network,
        ),
        Commands::P2shMultisig { required, pubkeys } => cmd_p2sh_multisig(required, &pubkeys, network),
        Commands::FetchUtxosAddress { address } => cmd_fetch_utxos_address(&address, network),
        Commands::BroadcastPublic { tx_hex } => cmd_broadcast_public(&tx_hex, network),
        Commands::Broadcast { tx_hex, rpc_url, rpc_user, rpc_pass } => {
            cmd_broadcast(&tx_hex, &rpc_url, rpc_user.as_deref(), rpc_pass.as_deref())
        }
        Commands::FetchUtxo { txid, vout, rpc_url, rpc_user, rpc_pass } => {
            cmd_fetch_utxo(&txid, vout, &rpc_url, rpc_user.as_deref(), rpc_pass.as_deref())
        }
        Commands::Demo => cmd_demo(network),
    }
}

/// Generate a new keypair and address
fn cmd_gen_key(network: Network) {
    println!("Doge-Hack: Generating New Keypair");
    println!("Network: {}", network);
    println!();

    let secp = Secp256k1::new();
    let mut secret_bytes = [0u8; 32];
    rand::thread_rng().fill(&mut secret_bytes);

    let secret_key = SecretKey::from_slice(&secret_bytes).expect("Valid secret key");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let address = DogeAddress::from_pubkey(&public_key, network);

    println!("Secret Key (hex): {}", hex::encode(secret_bytes));
    println!("Public Key: {}", public_key);
    println!("Address: {}", address.to_string());
}

/// Derive address from a secret key
fn cmd_address(secret_key_hex: &str, network: Network) {
    let secret_bytes = hex::decode(secret_key_hex).expect("Invalid hex secret key");
    let secret_key = SecretKey::from_slice(&secret_bytes).expect("Invalid secret key");
    
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let address = DogeAddress::from_pubkey(&public_key, network);

    println!("Network: {}", network);
    println!("Public Key: {}", public_key);
    println!("Address: {}", address.to_string());
}

/// Construct and sign a transaction
fn cmd_sign(
    txid: &str,
    vout: u32,
    secret_key_hex: &str,
    to_address: &str,
    amount: u64,
    fee: u64,
    change_address: Option<&str>,
    prev_value: Option<u64>,
    prev_script_hex: Option<&str>,
    rpc_url: Option<&str>,
    rpc_user: Option<&str>,
    rpc_pass: Option<&str>,
    api_key: Option<&str>,
    network: Network,
) {
    println!("Doge-Hack: Constructing Transaction");
    println!("Network: {}", network);
    println!();

    // Parse secret key
    let secret_bytes = hex::decode(secret_key_hex).expect("Invalid hex secret key");
    let secret_key = SecretKey::from_slice(&secret_bytes).expect("Invalid secret key");
    
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let from_address = DogeAddress::from_pubkey(&public_key, network);

    let to = match DogeAddress::from_base58(to_address) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("ERROR: invalid destination address: {e}");
            return;
        }
    };

    let change = match change_address {
        Some(s) => match DogeAddress::from_base58(s) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("ERROR: invalid change address: {e}");
                return;
            }
        },
        None => DogeAddress::from_pubkey(&public_key, network),
    };

    let (input_value, prev_script_hex) = match resolve_prevout(
        txid,
        vout,
        prev_value,
        prev_script_hex,
        rpc_url,
        rpc_user,
        rpc_pass,
        api_key,
        network,
    ) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: failed to resolve prevout: {e}");
            eprintln!("Hint: provide --rpc-url, or --api-key/CHAIN_SO_API_KEY, or --prev-value + --prev-script-hex.");
            return;
        }
    };

    println!("From: {}", from_address.to_string());
    println!("To: {}", to.to_string());
    println!("Input: {} sats", input_value);
    println!("Send: {} satoshis ({} DOGE)", amount, amount as f64 / 100_000_000.0);
    println!("Fee: {} satoshis ({} DOGE)", fee, fee as f64 / 100_000_000.0);
    println!();

    // Build transaction
    if input_value < amount.saturating_add(fee) {
        eprintln!("ERROR: insufficient input. need >= {} sats (amount+fee), got {} sats", amount + fee, input_value);
        return;
    }

    let change_value = input_value - amount - fee;

    let mut builder = TransactionBuilder::new();
    builder.add_input(txid, vout);
    builder.add_output(&to, amount);
    if change_value > 0 {
        builder.add_output(&change, change_value);
    }

    // Sign
    let prev_script_bytes = match hex::decode(&prev_script_hex) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("ERROR: invalid prev_script_hex: {e}");
            return;
        }
    };
    let prev_script_pubkey = bitcoin::ScriptBuf::from_bytes(prev_script_bytes);

    builder.sign_input(0, &secret_key, &prev_script_pubkey);

    let signed_tx = builder.build();
    let signed_tx_hex = serialize_hex(&signed_tx);

    println!("SIGNED Transaction Hex:");
    println!("{}", signed_tx_hex);

    if change_value > 0 {
        println!("Change: {} sats -> {}", change_value, change.to_string());
    }
}

fn resolve_prevout(
    txid: &str,
    vout: u32,
    prev_value: Option<u64>,
    prev_script_hex: Option<&str>,
    rpc_url: Option<&str>,
    rpc_user: Option<&str>,
    rpc_pass: Option<&str>,
    api_key: Option<&str>,
    network: Network,
) -> Result<(u64, String), Box<dyn std::error::Error>> {
    if let (Some(v), Some(s)) = (prev_value, prev_script_hex) {
        return Ok((v, s.to_string()));
    }

    if let Some(url) = rpc_url {
        let client = DogeRpcClient::new(url, rpc_user, rpc_pass);
        let utxo = client.fetch_utxo(txid, vout)?;
        return Ok((utxo.value, utxo.script_pubkey));
    }

    let key = match api_key {
        Some(k) => Some(k.to_string()),
        None => std::env::var("CHAIN_SO_API_KEY").ok(),
    };

    if let Some(k) = key {
        let client = SoChainV3Client::new(&k);
        let out = client.fetch_output(txid, vout, network)?;
        return Ok((out.value_satoshis, out.script_hex));
    }

    Err("no prevout source provided".into())
}

fn cmd_p2sh_multisig(required: u8, pubkeys_hex: &[String], network: Network) {
    if pubkeys_hex.is_empty() {
        eprintln!("ERROR: at least one --pubkey is required");
        return;
    }

    let mut pubkeys: Vec<Vec<u8>> = Vec::with_capacity(pubkeys_hex.len());
    for pk_hex in pubkeys_hex {
        match hex::decode(pk_hex) {
            Ok(b) => pubkeys.push(b),
            Err(e) => {
                eprintln!("ERROR: invalid pubkey hex '{pk_hex}': {e}");
                return;
            }
        }
    }

    let redeem = match script::multisig_redeem_script(required, &pubkeys) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR: failed to build redeem script: {e}");
            return;
        }
    };

    let h = script::redeem_script_hash160(&redeem);
    let addr = DogeAddress::from_script_hash(&h, network);

    println!("Network: {}", network);
    println!("Redeem Script (hex): {}", hex::encode(redeem.as_bytes()));
    println!("P2SH Address: {}", addr.to_string());
}

fn cmd_fetch_utxos_address(address: &str, network: Network) {
    println!("Doge-Hack: Fetching Address UTXOs (public API)");
    println!("Network: {}", network);
    println!("Address: {}", address);
    println!();

    let client = ChainSoClient::new();
    match client.get_tx_unspent(address, network) {
        Ok(utxos) => {
            if utxos.is_empty() {
                println!("No UTXOs found.");
                return;
            }

            println!("Found {} UTXO(s):", utxos.len());
            for u in utxos {
                println!("- {}:{} value={} sats conf={} scriptPubKey={}", u.txid, u.vout, u.value_satoshis, u.confirmations, u.script_hex);
            }
        }
        Err(e) => {
            eprintln!("ERROR: failed to fetch UTXOs: {e}");
        }
    }
}

fn cmd_broadcast_public(tx_hex: &str, network: Network) {
    println!("Doge-Hack: Broadcasting Transaction (public API)");
    println!("Network: {}", network);
    println!();

    let client = ChainSoClient::new();
    match client.send_tx(tx_hex, network) {
        Ok(txid) => {
            println!("SUCCESS: Transaction broadcast!");
            println!("TxID: {}", txid);
        }
        Err(e) => {
            eprintln!("ERROR: failed to broadcast transaction: {e}");
        }
    }
}

/// Broadcast a signed transaction via RPC
fn cmd_broadcast(tx_hex: &str, rpc_url: &str, rpc_user: Option<&str>, rpc_pass: Option<&str>) {
    println!("Doge-Hack: Broadcasting Transaction");
    println!("RPC URL: {}", rpc_url);
    println!();

    let client = DogeRpcClient::new(rpc_url, rpc_user, rpc_pass);
    
    match client.broadcast_tx(tx_hex) {
        Ok(result) => {
            println!("SUCCESS: Transaction broadcast!");
            println!("TxID: {}", result.txid);
        }
        Err(e) => {
            eprintln!("ERROR: Failed to broadcast transaction");
            eprintln!("{}", e);
        }
    }
}

/// Fetch UTXO information via RPC
fn cmd_fetch_utxo(txid: &str, vout: u32, rpc_url: &str, rpc_user: Option<&str>, rpc_pass: Option<&str>) {
    println!("Doge-Hack: Fetching UTXO");
    println!("TxID: {}", txid);
    println!("Vout: {}", vout);
    println!();

    let client = DogeRpcClient::new(rpc_url, rpc_user, rpc_pass);
    
    match client.fetch_utxo(txid, vout) {
        Ok(utxo) => {
            println!("UTXO Found:");
            println!("  Value: {} satoshis ({} DOGE)", utxo.value, utxo.value as f64 / 100_000_000.0);
            println!("  ScriptPubKey: {}", utxo.script_pubkey);
            println!("  Confirmations: {}", utxo.confirmations);
        }
        Err(e) => {
            eprintln!("ERROR: Failed to fetch UTXO");
            eprintln!("{}", e);
        }
    }
}

/// Demo mode - original behavior
fn cmd_demo(network: Network) {
    println!("Doge-Hack: Dogecoin Transaction Constructor Experiment");

    // Phase 1: Wallet
    println!("\n--- Phase 1: Wallet ---");
    
    let secp = Secp256k1::new();
    let mut secret_bytes = [0u8; 32];
    rand::thread_rng().fill(&mut secret_bytes);

    let secret_key = SecretKey::from_slice(&secret_bytes).expect("Valid secret key");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    println!("Secret Key: {}", secret_key.display_secret());
    println!("Public Key: {}", public_key);

    // Generate Doge Address
    let address = DogeAddress::from_pubkey(&public_key, network);
    let address_str = address.to_string();
    println!("Doge {} Address: {}", network, address_str);

    // Verify prefix
    let expected_prefix = match network {
        Network::Testnet => vec!['n', 'm'],
        Network::Mainnet => vec!['D'],
    };
    
    if expected_prefix.iter().any(|&p| address_str.starts_with(p)) {
        println!("SUCCESS: Address starts with expected prefix.");
    } else {
        println!("WARNING: Address prefix mismatch!");
    }

    // Phase 2: Mock Data
    println!("\n--- Phase 2: Mock Data ---");
    let mock_txid = "fb48f9e2068d0674c965e9057b6f87494df9278065a7f98ee591f7d3d7568553";
    let mock_vout = 0;
    println!("Mock UTXO: {} : {}", mock_txid, mock_vout);

    // Phase 3: Construction
    println!("\n--- Phase 3: Construction ---");
    
    let mut builder = TransactionBuilder::new();
    builder.add_input(mock_txid, mock_vout);

    let amount = 50 * 100_000_000;
    builder.add_output(&address, amount);

    let tx = builder.clone().build();
    println!("Transaction Constructed!");
    println!("Tx Version: {:?}", tx.version);
    println!("Tx LockTime: {:?}", tx.lock_time);
    println!("Tx Inputs: {}", tx.input.len());
    println!("Tx Outputs: {}", tx.output.len());

    let tx_hex = serialize_hex(&tx);
    println!("Unsigned Transaction Hex:\n{}", tx_hex);

    // Phase 4: Signing
    println!("\n--- Phase 4: Signing ---");

    let pubkey_hash = address.pubkey_hash();
    let prev_script_pubkey = bitcoin::script::Builder::new()
        .push_opcode(bitcoin::opcodes::all::OP_DUP)
        .push_opcode(bitcoin::opcodes::all::OP_HASH160)
        .push_slice(<&bitcoin::script::PushBytes>::try_from(pubkey_hash).unwrap())
        .push_opcode(bitcoin::opcodes::all::OP_EQUALVERIFY)
        .push_opcode(bitcoin::opcodes::all::OP_CHECKSIG)
        .into_script();

    println!("Signing Input 0...");
    builder.sign_input(0, &secret_key, &prev_script_pubkey);

    let signed_tx = builder.build();
    let signed_tx_hex = serialize_hex(&signed_tx);

    println!("SIGNED Transaction Hex:\n{}", signed_tx_hex);
    println!("\nSUCCESS: Transaction constructed and signed manually!");
}
