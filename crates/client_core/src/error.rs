use std::fmt;

#[derive(Debug)]
pub enum ClientError {
    Io(std::io::Error),
    Crypto(clef_crypto::CryptoError),
    InvalidIdentityFile,
}

impl From<std::io::Error> for ClientError {
    fn from(e: std::io::Error) -> Self {
        ClientError::Io(e)
    }
}

impl From<clef_crypto::CryptoError> for ClientError {
    fn from(e: clef_crypto::CryptoError) -> Self {
        ClientError::Crypto(e)
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::Io(e) => write!(f, "io error: {}", e),
            ClientError::Crypto(e) => write!(f, "crypto error: {}", e),
            ClientError::InvalidIdentityFile => write!(f, "invalid identity file"),
        }
    }
}

impl std::error::Error for ClientError {}