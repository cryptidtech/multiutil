use crate::{error::BaseEncoderError, prelude::Base};
use base58::{FromBase58, ToBase58};

/// a trait for base encoding implementations
pub trait BaseEncoder {
    /// convert a &[u8] to a base encoded value
    fn to_base_encoded(base: Base, b: &[u8]) -> String;

    /// convert a base encoded value to a Vec<u8>
    fn from_base_encoded(s: &str) -> Result<(Base, Vec<u8>), BaseEncoderError>;
}

/// a multibase encoder implementation for use as the default encoder
pub struct MultibaseEncoder {}

impl BaseEncoder for MultibaseEncoder {
    fn to_base_encoded(base: Base, b: &[u8]) -> String {
        multibase::encode(base, b)
    }
    fn from_base_encoded(s: &str) -> Result<(Base, Vec<u8>), BaseEncoderError> {
        Ok(multibase::decode(s)?)
    }
}

/// a bare Base58Btc encoder implementation for use with legacy Cids
pub struct Base58Encoder {}

impl BaseEncoder for Base58Encoder {
    fn to_base_encoded(_base: Base, b: &[u8]) -> String {
        b.to_base58()
    }
    fn from_base_encoded(s: &str) -> Result<(Base, Vec<u8>), BaseEncoderError> {
        match s.from_base58() {
            Ok(v) => Ok((Base::Base58Btc, v)),
            Err(e) => Err(BaseEncoderError::Base58(format!("{:?}", e))),
        }
    }
}
