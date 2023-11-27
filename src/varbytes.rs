use crate::prelude::{Base, BaseEncoded, EncodingInfo, Error};
use multitrait::prelude::{EncodeInto, TryDecodeFrom};

/// A wrapper type to handle serde of byte arrays as bytes
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Varbytes(pub Vec<u8>);

/// type alias for a Varbytes base encoded to/from string
pub type EncodedVarbytes = BaseEncoded<Varbytes>;

impl Varbytes {
    /// create an encoded varbytes
    pub fn encoded_new(v: Vec<u8>) -> EncodedVarbytes {
        BaseEncoded::new(Varbytes(v.clone()))
    }

    /// consume self and return inner vec
    pub fn to_inner(self) -> Vec<u8> {
        self.0
    }
}

impl EncodingInfo for Varbytes {
    fn preferred_encoding() -> Base {
        Base::Base16Lower
    }

    fn encoding(&self) -> Base {
        Base::Base16Lower
    }
}

impl Into<Vec<u8>> for Varbytes {
    fn into(self) -> Vec<u8> {
        let mut v = self.0.len().encode_into();
        v.append(&mut self.0.clone());
        v
    }
}

impl<'a> TryFrom<&'a [u8]> for Varbytes {
    type Error = Error;

    fn try_from(s: &'a [u8]) -> Result<Self, Error> {
        let (len, ptr) = usize::try_decode_from(s)?;
        let v = ptr[..len].to_vec();
        Ok(Varbytes(v))
    }
}
