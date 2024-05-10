// SPDX-License-Idnetifier: Apache-2.0
use crate::{
    base_name,
    error::{BaseEncodedError, BaseEncoderError},
    prelude::Base,
    Error,
};
use base58::{FromBase58, ToBase58};
use base64::prelude::*;

/// a trait for base encoding implementations
pub trait BaseEncoder {
    /// convert a &[u8] to a base encoded value
    fn to_base_encoded(base: Base, b: &[u8]) -> String;

    /// convert a base encoded value to a Vec<u8>
    fn from_base_encoded(s: &str) -> Result<(Base, Vec<u8>), Error>;

    /// get the debug string for the given base
    fn debug_string(base: Base) -> String;

    /// get the preferred base encoding for this encoder
    fn preferred_encoding(base: Base) -> Base;
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
    fn preferred_encoding(base: Base) -> Base {
        base
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
        format!("{} ('{}')", base_name(Base::Base58Btc), Base::Base58Btc.code())
    }
    fn preferred_encoding(_base: Base) -> Base {
        Base::Base58Btc
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

        // next try "naked" encoding in increasing symbol space size order
        
        // base16
        if let Ok(data) = hex::decode(s) {
            return Ok((Base::Base16Lower, data))
        }

        // base32 (no padding)
        if let Some(data) = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, s) {
            return Ok((Base::Base32Upper, data))
        }

        // base32 (padding)
        if let Some(data) = base32::decode(base32::Alphabet::Rfc4648 { padding: true }, s) {
            return Ok((Base::Base32PadUpper, data))
        }

        // base32 (lower + no padding)
        if let Some(data) = base32::decode(base32::Alphabet::Rfc4648Lower { padding: false }, s) {
            return Ok((Base::Base32Lower, data))
        }

        // base32 (padding)
        if let Some(data) = base32::decode(base32::Alphabet::Rfc4648Lower { padding: true }, s) {
            return Ok((Base::Base32PadLower, data))
        }

        // base32 (no padding)
        if let Some(data) = base32::decode(base32::Alphabet::Rfc4648Hex { padding: false }, s) {
            return Ok((Base::Base32HexUpper, data))
        }

        // base32 (padding)
        if let Some(data) = base32::decode(base32::Alphabet::Rfc4648Hex { padding: true }, s) {
            return Ok((Base::Base32HexPadUpper, data))
        }

        // base32 (lower + no padding)
        if let Some(data) = base32::decode(base32::Alphabet::Rfc4648HexLower { padding: false }, s) {
            return Ok((Base::Base32HexLower, data))
        }

        // base32 (padding)
        if let Some(data) = base32::decode(base32::Alphabet::Rfc4648HexLower { padding: true }, s) {
            return Ok((Base::Base32HexPadLower, data))
        }

        // base36
        if let Ok(data) = base36::decode(s) {
            return Ok((Base::Base36Lower, data))
        }

        // base58 (bitcoin)
        if let Ok(data) = s.from_base58() {
            return Ok((Base::Base58Btc, data));
        }

        // base64 (no padding)
        if let Ok(data) = BASE64_STANDARD_NO_PAD.decode(s) {
            return Ok((Base::Base64, data))
        }

        // base64 (padding)
        if let Ok(data) = BASE64_STANDARD.decode(s) {
            return Ok((Base::Base64Pad, data))
        }

        // base64 (url + no padding)
        if let Ok(data) = BASE64_URL_SAFE_NO_PAD.decode(s) {
            return Ok((Base::Base64Pad, data))
        }

        // base64 (url + padding)
        if let Ok(data) = BASE64_URL_SAFE.decode(s) {
            return Ok((Base::Base64Pad, data))
        }

        // raise an error
        Err(BaseEncodedError::ValueFailed.into())
    }
    fn debug_string(base: Base) -> String {
        format!("{} ('{}')", base_name(base), base.code())
    }
    fn preferred_encoding(base: Base) -> Base {
        base
    }
}
