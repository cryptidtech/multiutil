// SPDX-License-Idnetifier: Apache-2.0
use crate::{
    base_name,
    error::{BaseEncodedError, BaseEncoderError},
    prelude::Base,
    Error,
};

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
        Base::Base58Btc.encode(b)
    }
    fn from_base_encoded(s: &str) -> Result<(Base, Vec<u8>), Error> {
        match Base::Base58Btc.decode(s) {
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
        if let Ok(data) = Base::Base2.decode(s) {
            return Ok((Base::Base2, data))
        } else if let Ok(data) = Base::Base8.decode(s) {
            return Ok((Base::Base8, data))
        } else if let Ok(data) = Base::Base10.decode(s) {
            return Ok((Base::Base10, data))
        } else if let Ok(data) = Base::Base16Lower.decode(s) {
            return Ok((Base::Base16Lower, data))
        } else if let Ok(data) = Base::Base16Upper.decode(s) {
            return Ok((Base::Base16Upper, data))
        } else if let Ok(data) = Base::Base32Lower.decode(s) {
            return Ok((Base::Base32Lower, data))
        } else if let Ok(data) = Base::Base32Upper.decode(s) {
            return Ok((Base::Base32Upper, data))
        } else if let Ok(data) = Base::Base32PadLower.decode(s) {
            return Ok((Base::Base32PadLower, data))
        } else if let Ok(data) = Base::Base32PadUpper.decode(s) {
            return Ok((Base::Base32PadUpper, data))
        } else if let Ok(data) = Base::Base32HexLower.decode(s) {
            return Ok((Base::Base32HexLower, data))
        } else if let Ok(data) = Base::Base32HexUpper.decode(s) {
            return Ok((Base::Base32HexUpper, data))
        } else if let Ok(data) = Base::Base32HexPadLower.decode(s) {
            return Ok((Base::Base32HexPadLower, data))
        } else if let Ok(data) = Base::Base32HexPadUpper.decode(s) {
            return Ok((Base::Base32HexPadUpper, data))
        } else if let Ok(data) = Base::Base32Z.decode(s) {
            return Ok((Base::Base32Z, data))
        } else if let Ok(data) = Base::Base36Lower.decode(s) {
            return Ok((Base::Base36Lower, data))
        } else if let Ok(data) = Base::Base36Upper.decode(s) {
            return Ok((Base::Base36Upper, data))
        } else if let Ok(data) = Base::Base58Flickr.decode(s) {
            return Ok((Base::Base58Flickr, data))
        } else if let Ok(data) = Base::Base58Btc.decode(s) {
            return Ok((Base::Base58Btc, data))
        } else if let Ok(data) = Base::Base64.decode(s) {
            return Ok((Base::Base64, data))
        } else if let Ok(data) = Base::Base64Pad.decode(s) {
            return Ok((Base::Base64Pad, data))
        } else if let Ok(data) = Base::Base64Url.decode(s) {
            return Ok((Base::Base64Url, data))
        } else if let Ok(data) = Base::Base64UrlPad.decode(s) {
            return Ok((Base::Base64UrlPad, data))
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
