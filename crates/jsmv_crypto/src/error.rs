use derive_more::{Display, Error, From};

use mavryk_crypto_rs::{
    base58::FromBase58CheckError,
    hash::{FromBytesError, TryFromPKError},
    CryptoError,
};

#[derive(Display, Debug, Error, From)]
pub enum Error {
    MavrykFromBase58Error { source: FromBase58CheckError },
    MavrykFromBytesError { source: FromBytesError },
    MavrykTryFromPKError { source: TryFromPKError },
    MavrykCryptoError { source: CryptoError },
    InvalidSignature,
}

pub type Result<T> = std::result::Result<T, Error>;
