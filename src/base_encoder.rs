use crate::{
    base_name,
    error::{BaseEncodedError, BaseEncoderError},
    prelude::Base,
    Error,
};
use base58::{FromBase58, ToBase58};

/// a trait for base encoding implementations
pub trait BaseEncoder {
    /// convert a &[u8] to a base encoded value
    fn to_base_encoded(base: Base, b: &[u8]) -> String;

    /// convert a base encoded value to a Vec<u8>
    fn from_base_encoded(s: &str) -> Result<(Base, Vec<u8>), Error>;

    /// get the debug string for the given base
    fn debug_string(base: Base) -> String;
}

/// a multibase encoder implementation for use as the default encoder
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultibaseEncoder {}

impl BaseEncoder for MultibaseEncoder {
    fn to_base_encoded(base: Base, b: &[u8]) -> String {
        multibase::encode(base, b)
    }
    fn from_base_encoded(s: &str) -> Result<(Base, Vec<u8>), Error> {
        Ok(multibase::decode(s).map_err(|_| BaseEncodedError::ValueFailed)?)
    }
    fn debug_string(base: Base) -> String {
        format!("{} ('{}')", base_name(base), base.code())
    }
}

/// a bare Base58Btc encoder implementation for use with legacy Cids
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Base58Encoder {}

impl BaseEncoder for Base58Encoder {
    fn to_base_encoded(_base: Base, b: &[u8]) -> String {
        b.to_base58()
    }
    fn from_base_encoded(s: &str) -> Result<(Base, Vec<u8>), Error> {
        match s.from_base58() {
            Ok(v) => Ok((Base::Base58Btc, v)),
            Err(e) => Err(BaseEncoderError::Base58(format!("{:?}", e)).into()),
        }
    }
    fn debug_string(_base: Base) -> String {
        "base58Btc".to_string()
    }
}

/// a speculative encoder that tries to detect the correct encoding and decode it
/// encoding is always done using multibase so this does not support symetric 
/// decode/encode round trips. this is useful for decoding CIDs that might be 
/// base58 encoded "legacy" CIDs but alsy may be multibase encoded CIDs.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DetectedEncoder {}

impl BaseEncoder for DetectedEncoder {
    fn to_base_encoded(base: Base, b: &[u8]) -> String {
        multibase::encode(base, b)
    }
    fn from_base_encoded(s: &str) -> Result<(Base, Vec<u8>), Error> {
        // first try multibase decoding
        if let Ok((base, data)) = multibase::decode(s) {
            return Ok((base, data));
        }
        // next try base58 encoding
        if let Ok(data) = s.from_base58() {
            return Ok((Base::Base58Btc, data));
        }
        // raise an error
        Err(BaseEncodedError::ValueFailed.into())
    }
    fn debug_string(base: Base) -> String {
        format!("{} ('{}')", base_name(base), base.code())
    }
}
