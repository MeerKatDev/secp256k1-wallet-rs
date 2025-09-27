use std::fmt;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    /// e.g. Ethereum
    #[default]
    Ecdsa,
    /// e.g. Solana
    Eddsa,
}

impl From<String> for KeyType {
    fn from(s: String) -> Self {
        KeyType::from_str(&s).unwrap() // or handle error properly
    }
}

impl From<KeyType> for String {
    fn from(k: KeyType) -> Self {
        k.to_string()
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyType::Ecdsa => write!(f, "ecdsa"),
            KeyType::Eddsa => write!(f, "eddsa"),
        }
    }
}

impl FromStr for KeyType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ecdsa" => Ok(KeyType::Ecdsa),
            "eddsa" => Ok(KeyType::Eddsa),
            other => Err(format!("Unknown key type: {other}")),
        }
    }
}
