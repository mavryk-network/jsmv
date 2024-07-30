use mavryk_crypto_rs::PublicKeySignatureVerifier;
use serde::{Deserialize, Serialize};

use crate::{public_key::PublicKey, Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Signature {
    Ed25519(mavryk_crypto_rs::hash::Signature),
}

impl Signature {
    pub fn verify(&self, public_key: &PublicKey, message: &[u8]) -> Result<()> {
        match (self, public_key) {
            (Signature::Ed25519(sig), PublicKey::Ed25519(pk)) => {
                let result = pk.verify_signature(sig, message).unwrap();
                if result {
                    Ok(())
                } else {
                    Err(Error::InvalidSignature)
                }
            }
        }
    }
}
