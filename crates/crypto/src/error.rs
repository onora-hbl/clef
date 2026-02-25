use std::fmt;

#[derive(Debug)]
pub enum CryptoError {
    InvalidKeyMaterial,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::InvalidKeyMaterial => write!(f, "invalid key material"),
        }
    }
}

impl std::error::Error for CryptoError {}