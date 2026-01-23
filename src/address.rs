use bitcoin::secp256k1::PublicKey;
use bitcoin::hashes::{sha256, ripemd160, Hash};
use bitcoin::base58;
use std::fmt;

use crate::network::Network;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressKind {
    P2pkh,
    P2sh,
}

#[derive(Debug)]
pub enum AddressError {
    InvalidBase58Check(String),
    InvalidLength(usize),
    UnknownVersionByte(u8),
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressError::InvalidBase58Check(e) => write!(f, "invalid base58check: {e}"),
            AddressError::InvalidLength(n) => write!(f, "invalid payload length: {n}, expected 21"),
            AddressError::UnknownVersionByte(b) => write!(f, "unknown version byte: 0x{b:02x}"),
        }
    }
}

impl std::error::Error for AddressError {}

/// Scaffolding for Dogecoin Address generation
/// 
/// Dogecoin addresses use different prefixes based on network:
/// - Testnet P2PKH: 'n' or 'm' (version byte 0x71)
/// - Mainnet P2PKH: 'D' (version byte 0x1E)
pub struct DogeAddress {
    pub payload: Vec<u8>,
    pub network: Network,
}

impl DogeAddress {
    /// Create a new DogeAddress from a public key with network configuration
    pub fn from_pubkey(public_key: &PublicKey, network: Network) -> Self {
        // 1. Serialize Public Key (Compressed)
        let pk_bytes = public_key.serialize();

        // 2. SHA256(PublicKey)
        let sha_hash = sha256::Hash::hash(&pk_bytes);

        // 3. RIPEMD160(SHA256(PublicKey))
        let ripemd_hash = ripemd160::Hash::hash(sha_hash.as_byte_array());

        // 4. Prepend Network Byte
        let version_byte = network.p2pkh_version_byte();
        let mut payload = Vec::with_capacity(21);
        payload.push(version_byte);
        payload.extend_from_slice(ripemd_hash.as_byte_array());

        Self { payload, network }
    }

    /// Create a P2PKH address from a 20-byte pubkey hash
    pub fn from_pubkey_hash(pubkey_hash20: &[u8; 20], network: Network) -> Self {
        let mut payload = Vec::with_capacity(21);
        payload.push(network.p2pkh_version_byte());
        payload.extend_from_slice(pubkey_hash20);
        Self { payload, network }
    }

    /// Create a P2SH address from a 20-byte script hash (HASH160(redeem_script))
    pub fn from_script_hash(script_hash20: &[u8; 20], network: Network) -> Self {
        let mut payload = Vec::with_capacity(21);
        payload.push(network.p2sh_version_byte());
        payload.extend_from_slice(script_hash20);
        Self { payload, network }
    }

    /// Parse a Base58Check-encoded Dogecoin address and infer network/kind via version byte.
    pub fn from_base58(s: &str) -> Result<Self, AddressError> {
        let decoded = base58::decode_check(s).map_err(|e| AddressError::InvalidBase58Check(e.to_string()))?;
        if decoded.len() != 21 {
            return Err(AddressError::InvalidLength(decoded.len()));
        }

        let version = decoded[0];
        let network = if version == Network::Testnet.p2pkh_version_byte() || version == Network::Testnet.p2sh_version_byte() {
            Network::Testnet
        } else if version == Network::Mainnet.p2pkh_version_byte() || version == Network::Mainnet.p2sh_version_byte() {
            Network::Mainnet
        } else {
            return Err(AddressError::UnknownVersionByte(version));
        };

        Ok(Self {
            payload: decoded,
            network,
        })
    }

    pub fn kind(&self) -> AddressKind {
        let version = self.payload[0];
        if version == self.network.p2pkh_version_byte() {
            AddressKind::P2pkh
        } else if version == self.network.p2sh_version_byte() {
            AddressKind::P2sh
        } else {
            // This should not happen if the address was constructed/parsing correctly.
            // Default to P2PKH to preserve behavior.
            AddressKind::P2pkh
        }
    }

    /// Return the 20-byte HASH160 embedded in the address (pubkey-hash for P2PKH, script-hash for P2SH).
    pub fn hash160(&self) -> &[u8] {
        &self.payload[1..21]
    }

    /// Create a new DogeAddress for Testnet (convenience method)
    pub fn from_pubkey_testnet(public_key: &PublicKey) -> Self {
        Self::from_pubkey(public_key, Network::Testnet)
    }

    /// Create a new DogeAddress for Mainnet (convenience method)
    pub fn from_pubkey_mainnet(public_key: &PublicKey) -> Self {
        Self::from_pubkey(public_key, Network::Mainnet)
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

    #[test]
    fn test_doge_address_testnet_prefix() {
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let secret_key = bitcoin::secp256k1::SecretKey::from_slice(&b"12345678901234567890123456789012"[..]).unwrap();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        let address = DogeAddress::from_pubkey(&public_key, Network::Testnet);
        let s = address.to_string();

        assert!(s.starts_with('n') || s.starts_with('m'), "Testnet address {} should start with n or m", s);
    }

    #[test]
    fn test_doge_address_mainnet_prefix() {
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let secret_key = bitcoin::secp256k1::SecretKey::from_slice(&b"12345678901234567890123456789012"[..]).unwrap();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        let address = DogeAddress::from_pubkey(&public_key, Network::Mainnet);
        let s = address.to_string();

        assert!(s.starts_with('D'), "Mainnet address {} should start with D", s);
    }

    #[test]
    fn test_convenience_methods() {
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let secret_key = bitcoin::secp256k1::SecretKey::from_slice(&b"12345678901234567890123456789012"[..]).unwrap();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        let testnet_addr = DogeAddress::from_pubkey_testnet(&public_key);
        let mainnet_addr = DogeAddress::from_pubkey_mainnet(&public_key);

        assert!(testnet_addr.to_string().starts_with('n') || testnet_addr.to_string().starts_with('m'));
        assert!(mainnet_addr.to_string().starts_with('D'));
    }

    #[test]
    fn test_parse_roundtrip_p2pkh() {
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let secret_key = bitcoin::secp256k1::SecretKey::from_slice(&b"12345678901234567890123456789012"[..]).unwrap();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let address = DogeAddress::from_pubkey(&public_key, Network::Testnet);
        let s = address.to_string();

        let parsed = DogeAddress::from_base58(&s).unwrap();
        assert_eq!(parsed.network, Network::Testnet);
        assert_eq!(parsed.kind(), AddressKind::P2pkh);
        assert_eq!(parsed.payload, address.payload);
    }

    #[test]
    fn test_p2sh_prefix_bytes() {
        let hash = [0x11u8; 20];
        let a_test = DogeAddress::from_script_hash(&hash, Network::Testnet);
        let a_main = DogeAddress::from_script_hash(&hash, Network::Mainnet);
        assert_eq!(a_test.payload[0], Network::Testnet.p2sh_version_byte());
        assert_eq!(a_main.payload[0], Network::Mainnet.p2sh_version_byte());
    }
}
