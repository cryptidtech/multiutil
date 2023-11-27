use crate::prelude::{Base, BaseEncoded, EncodingInfo, Error};
use core::fmt;
use multitrait::prelude::{EncodeInto, TryDecodeFrom};

/// A wrapper type to handle serde of numeric types as varuint bytes
#[derive(Clone, PartialEq)]
pub struct Varuint<T>(pub T);

/// type alias for a Varuint base encoded to/from string
pub type EncodedVaruint<T> = BaseEncoded<Varuint<T>>;

impl<T> Varuint<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    /// create a new encoded varuint
    pub fn encoded_new(t: T) -> EncodedVaruint<T> {
        BaseEncoded::new(Self(t))
    }

    /// consume self and return inner value
    pub fn to_inner(self) -> T {
        self.0
    }
}

impl<T> Default for Varuint<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(T::default())
    }
}

impl<T> fmt::Debug for Varuint<T>
where
    T: EncodeInto,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0.encode_into().as_slice())
    }
}

impl<T> EncodingInfo for Varuint<T> {
    fn preferred_encoding() -> Base {
        Base::Base16Lower
    }

    fn encoding(&self) -> Base {
        Base::Base16Lower
    }
}

impl<T> Into<Vec<u8>> for Varuint<T>
where
    T: EncodeInto,
{
    fn into(self) -> Vec<u8> {
        self.0.encode_into()
    }
}

impl<'a, T> TryFrom<&'a [u8]> for Varuint<T>
where
    T: TryDecodeFrom<'a>,
{
    type Error = Error;

    fn try_from(s: &'a [u8]) -> Result<Self, Error> {
        let (t, _) =
            T::try_decode_from(s).map_err(|_| Error::custom("failed to decode varuint"))?;
        Ok(Varuint(t))
    }
}
