use std::{fs, path::Path};
use clef_crypto::IdentityKeypair;

use crate::ClientError;

pub fn save_identity(path: &Path, identity: &IdentityKeypair) -> Result<(), ClientError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let bytes = identity.to_bytes();

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)?;
        use std::io::Write;
        file.write_all(&bytes)?;
    }

    #[cfg(not(unix))]
    {
        fs::write(path, bytes)?;
    }

    Ok(())
}

pub fn load_identity(path: &Path) -> Result<IdentityKeypair, ClientError> {
    let bytes = fs::read(path)?;

    if bytes.len() != 32 {
        return Err(ClientError::InvalidIdentityFile);
    }

    let array: [u8; 32] = bytes.try_into().map_err(|_| ClientError::InvalidIdentityFile)?;
    Ok(IdentityKeypair::from_bytes(array)?)
}