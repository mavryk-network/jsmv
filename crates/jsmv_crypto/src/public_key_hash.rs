use std::fmt;

use boa_gc::{empty_trace, Finalize, Trace};
use serde::{Deserialize, Serialize};
use mavryk_crypto_rs::{
    blake2b::digest,
    hash::{ContractMv1Hash, HashTrait},
    PublicKeyWithHash,
};

use crate::{
    error::{Error, Result},
    public_key::PublicKey,
};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Finalize,
)]
pub enum PublicKeyHash {
    Mv1(ContractMv1Hash),
}

unsafe impl Trace for PublicKeyHash {
    empty_trace!();
}

impl PublicKeyHash {
    pub fn to_base58(&self) -> String {
        let PublicKeyHash::Mv1(tz1) = self;
        tz1.to_base58_check()
    }

    pub fn from_base58(data: &str) -> Result<Self> {
        let tz1 = ContractMv1Hash::from_base58_check(data)?;
        Ok(PublicKeyHash::Mv1(tz1))
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let tz1 = ContractMv1Hash::try_from_bytes(bytes)?;
        Ok(PublicKeyHash::Mv1(tz1))
    }

    pub fn as_bytes(&self) -> &[u8] {
        let PublicKeyHash::Mv1(tz1) = self;
        &tz1.0
    }
    pub fn digest(data: &[u8]) -> Result<Self> {
        let out_len = ContractMv1Hash::hash_size();
        let bytes = digest(data, out_len).expect("failed to create hash");
        Self::from_slice(&bytes)
    }
}

impl fmt::Display for PublicKeyHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

impl TryFrom<&PublicKey> for PublicKeyHash {
    type Error = Error;

    fn try_from(pk: &PublicKey) -> Result<Self> {
        let PublicKey::Ed25519(key) = pk;
        let tz1 = key.pk_hash()?;
        Ok(PublicKeyHash::Mv1(tz1))
    }
}
