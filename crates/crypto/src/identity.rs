use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use rand::rngs::OsRng;

use crate::CryptoError;

#[derive(Clone)]
pub struct IdentityPublicKey(VerifyingKey);

impl IdentityPublicKey {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }
}

pub struct IdentityKeypair {
    signing_key: SigningKey,
}

impl IdentityKeypair {
     pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Self { signing_key }
    }

    pub fn public_key(&self) -> IdentityPublicKey {
        IdentityPublicKey(self.signing_key.verifying_key())
    }

    pub fn sign(&self, data: &[u8]) -> Signature {
        self.signing_key.sign(data)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Result<Self, CryptoError> {
        Ok(Self {
            signing_key: SigningKey::from_bytes(&bytes),
        })
    }
}