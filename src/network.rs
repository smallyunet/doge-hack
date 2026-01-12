use std::fmt;
use std::str::FromStr;

/// Dogecoin Network Configuration
/// 
/// Provides network-specific parameters for address encoding.
/// - Testnet: Addresses start with 'n' or 'm' (version byte 0x71)
/// - Mainnet: Addresses start with 'D' (version byte 0x1E)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Network {
    #[default]
    Testnet,
    Mainnet,
}

impl Network {
    /// Get the version byte for P2PKH addresses
    pub fn p2pkh_version_byte(&self) -> u8 {
        match self {
            Network::Testnet => 0x71, // 'n' or 'm' prefix
            Network::Mainnet => 0x1E, // 'D' prefix
        }
    }

    /// Get the version byte for P2SH addresses (for future use)
    pub fn p2sh_version_byte(&self) -> u8 {
        match self {
            Network::Testnet => 0xC4, // '2' prefix
            Network::Mainnet => 0x16, // '9' or 'A' prefix
        }
    }

    /// Get the WIF (Wallet Import Format) version byte
    pub fn wif_version_byte(&self) -> u8 {
        match self {
            Network::Testnet => 0xF1, // WIF testnet
            Network::Mainnet => 0x9E, // WIF mainnet
        }
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Network::Testnet => write!(f, "testnet"),
            Network::Mainnet => write!(f, "mainnet"),
        }
    }
}

impl FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "testnet" | "test" => Ok(Network::Testnet),
            "mainnet" | "main" => Ok(Network::Mainnet),
            _ => Err(format!("Unknown network: {}. Use 'testnet' or 'mainnet'", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_version_bytes() {
        assert_eq!(Network::Testnet.p2pkh_version_byte(), 0x71);
        assert_eq!(Network::Mainnet.p2pkh_version_byte(), 0x1E);
    }

    #[test]
    fn test_network_from_str() {
        assert_eq!(Network::from_str("testnet").unwrap(), Network::Testnet);
        assert_eq!(Network::from_str("mainnet").unwrap(), Network::Mainnet);
        assert_eq!(Network::from_str("MAINNET").unwrap(), Network::Mainnet);
        assert!(Network::from_str("invalid").is_err());
    }

    #[test]
    fn test_network_display() {
        assert_eq!(format!("{}", Network::Testnet), "testnet");
        assert_eq!(format!("{}", Network::Mainnet), "mainnet");
    }
}
