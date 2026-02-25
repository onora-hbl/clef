use std::path::Path;

use clef_crypto::{IdentityKeypair, IdentityPublicKey};
use crate::{storage, ClientError};

pub struct Client {
    identity: IdentityKeypair,
}

impl Client {
    pub fn create_new(identity_path: &Path) -> Result<Self, ClientError> {
        let identity = IdentityKeypair::generate();
        storage::save_identity(identity_path, &identity)?;
        Ok(Self { identity })
    }

    pub fn load(identity_path: &Path) -> Result<Self, ClientError> {
        let identity = storage::load_identity(identity_path)?;
        Ok(Self { identity })
    }

    pub fn identity_public_key(&self) -> IdentityPublicKey {
        self.identity.public_key()
    }
}