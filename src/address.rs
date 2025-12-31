use bitcoin::secp256k1::PublicKey;
use bitcoin::hashes::{sha256, ripemd160, Hash};
use bitcoin::base58;

/// Scaffolding for Dogecoin Address generation
/// 
/// Dogecoin Testnet P2PKH start with 'n' or 'm' (113 decimal = 0x71)
pub struct DogeAddress {
    pub payload: Vec<u8>,
}

impl DogeAddress {
    /// Create a new DogeAddress from a public key
    pub fn from_pubkey(public_key: &PublicKey) -> Self {
        // 1. Serialize Public Key (Compressed)
        let pk_bytes = public_key.serialize();

        // 2. SHA256(PublicKey)
        let sha_hash = sha256::Hash::hash(&pk_bytes);

        // 3. RIPEMD160(SHA256(PublicKey))
        let ripemd_hash = ripemd160::Hash::hash(sha_hash.as_byte_array());

        // 4. Prepend Network Byte (0x71 for Dogecoin Testnet)
        let mut payload = Vec::with_capacity(21);
        payload.push(0x71);
        payload.extend_from_slice(ripemd_hash.as_byte_array());

        Self { payload }
    }

    /// Extract the PubKeyHash (20 bytes) from the address
    pub fn pubkey_hash(&self) -> &[u8] {
        // [0] is header, [1..21] is hash
        &self.payload[1..21]
    }

    /// Return the Base58Check encoded string
    pub fn to_string(&self) -> String {
        // Use bitcoin's internal base58::encode_check if available, or manual simple encode
        // Since we want to use the library's primitives:
        // bitcoin::base58::encode_check takes (data) where data includes the prefix? 
        // usage: base58::encode_check(payload) usually does checksumming.
        // Let's check if we construct the full payload + checksum manually or use a helper.
        // bitcoin::base58::check_encode_slice(self.payload) 
        
        // Note: The `payload` field in our struct ALREADY includes the version byte (0x71).
        // Standard Base58Check is: [Version][Payload][Checksum]
        // `bitcoin::base58::check_encode_slice` usually takes the versioned payload and appends checksum.
        
        base58::encode_check(&self.payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_doge_address_prefix() {
        // Use a fixed secret key to ensure deterministic output if needed, 
        // or just check the property of the output string.
        let secp = bitcoin::secp256k1::Secp256k1::new();
        // Random key
        let secret_key = bitcoin::secp256k1::SecretKey::from_slice(&b"12345678901234567890123456789012"[..]).unwrap();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        let address = DogeAddress::from_pubkey(&public_key);
        let s = address.to_string();

        assert!(s.starts_with('n') || s.starts_with('m'), "Address {} should start with n or m", s);
    }
}
