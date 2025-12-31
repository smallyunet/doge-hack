use bitcoin::{Transaction, TxIn, TxOut, OutPoint, Txid, Sequence, ScriptBuf};
use bitcoin::opcodes::all::{OP_DUP, OP_HASH160, OP_EQUALVERIFY, OP_CHECKSIG};
use bitcoin::blockdata::script::Builder as ScriptBuilder;
use bitcoin::absolute::LockTime;
use bitcoin::amount::Amount;
use bitcoin::hashes::Hash;
use bitcoin::sighash::{SighashCache, EcdsaSighashType};
use bitcoin::secp256k1::{Secp256k1, SecretKey, Message};
use hex::FromHex;

use crate::address::DogeAddress;

/// Scaffolding for Dogecoin Transaction Construction
/// 
/// Dogecoin transactions are binary-compatible with Bitcoin transactions.
/// We use the standard bitcoin::Transaction struct but construct it manually.

#[derive(Clone)]
pub struct TransactionBuilder {
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self { 
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// Add a UTXO as input (Hardcoded for now in early phases)
    pub fn add_input(&mut self, txid_hex: &str, vout: u32) {
        let txid = Txid::from_str(txid_hex).expect("Invalid Hex Txid");
        let input = TxIn {
            previous_output: OutPoint { txid, vout },
            script_sig: ScriptBuf::new(), // Empty for now, will sign later
            sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
            witness: bitcoin::Witness::default(),
        };
        self.inputs.push(input);
    }

    /// Add an output to a destination address
    pub fn add_output(&mut self, address: &DogeAddress, amount_satoshis: u64) {
        // Build P2PKH Script: OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG
        let pubkey_hash = address.pubkey_hash();
        
        // Manual Script Construction
        let script_pubkey = ScriptBuilder::new()
            .push_opcode(OP_DUP)
            .push_opcode(OP_HASH160)
            .push_slice(<&bitcoin::script::PushBytes>::try_from(pubkey_hash).expect("valid push bytes")) // Push the 20-byte hash
            .push_opcode(OP_EQUALVERIFY)
            .push_opcode(OP_CHECKSIG)
            .into_script();

        let output = TxOut {
            value: Amount::from_sat(amount_satoshis),
            script_pubkey: script_pubkey,
        };
        self.outputs.push(output);
    }

    /// Build the final transaction
    pub fn build(self) -> Transaction {
        Transaction {
            version: bitcoin::transaction::Version::ONE, // Dogecoin uses Version 1 usually
            lock_time: LockTime::ZERO,
            input: self.inputs,
            output: self.outputs,
        }
    }

    /// Sign a specific input (Classic P2PKH)
    /// WARNING: This modifies the `inputs` in place.
    pub fn sign_input(
        &mut self, 
        input_index: usize, 
        secret_key: &SecretKey, 
        previous_script_pubkey: &ScriptBuf
    ) {
        let secp = Secp256k1::new();
        let public_key = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, secret_key);

        // 1. Create the transaction to sign
        // We need a temporary transaction structure because SighashCache borrows it
        let mut tx = self.to_transaction_ref();

        // 2. Calculate Sighash
        let mut sighash_cache = SighashCache::new(&mut tx);
        let sighash = sighash_cache
            .legacy_signature_hash(
                input_index, 
                previous_script_pubkey, 
                EcdsaSighashType::All.to_u32()
            )
            .expect("Sighash generation failed");

        // 3. Sign the Hash
        let message = Message::from_digest(sighash.to_byte_array());
        let signature = secp.sign_ecdsa(&message, secret_key);
        
        // 4. Construct ScriptSig: <Sig> <PubKey>
        let mut sig_with_hashtype = signature.serialize_der().to_vec();
        sig_with_hashtype.push(EcdsaSighashType::All.to_u32() as u8); // Append SIGHASH_ALL (0x01)

        let script_sig = ScriptBuilder::new()
            .push_slice(<&bitcoin::script::PushBytes>::try_from(sig_with_hashtype.as_slice()).unwrap())
            .push_slice(<&bitcoin::script::PushBytes>::try_from(public_key.serialize().as_slice()).unwrap())
            .into_script();

        // 5. Update Input
        self.inputs[input_index].script_sig = script_sig;
    }

    // Helper to create a transaction reference for SighashCache
    fn to_transaction_ref(&self) -> Transaction {
        Transaction {
            version: bitcoin::transaction::Version::ONE,
            lock_time: LockTime::ZERO,
            input: self.inputs.clone(),
            output: self.outputs.clone(),
        }
    }
}



use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
    use crate::address::DogeAddress;

    #[test]
    fn test_transaction_structure() {
        let mut builder = TransactionBuilder::new();
        let txid = "fb48f9e2068d0674c965e9057b6f87494df9278065a7f98ee591f7d3d7568553";
        builder.add_input(txid, 0);

        // Dummy address
        let secp = Secp256k1::new();
        let secret = SecretKey::from_slice(&b"12345678901234567890123456789012"[..]).unwrap();
        let pubkey = PublicKey::from_secret_key(&secp, &secret);
        let address = DogeAddress::from_pubkey(&pubkey);

        builder.add_output(&address, 1000);

        let tx = builder.build();
        assert_eq!(tx.input.len(), 1);
        assert_eq!(tx.output.len(), 1);
        assert_eq!(tx.output[0].value.to_sat(), 1000);
    }
}
