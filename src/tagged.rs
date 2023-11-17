use crate::prelude::{Codec, EncodeInto, Error, TryDecodeFrom};
use core::{fmt, ops};

/// Smart pointer for multicodec tagged data
pub struct Tagged<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    /// Multicoded codec
    pub codec: Codec,
    t: T,
}

impl<T> Tagged<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    /// Construct a Tagged smart pointer with the given multicodec codec
    pub fn new(codec: Codec, t: T) -> Self {
        Self { codec, t }
    }
}

impl<T> EncodeInto for Tagged<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    fn encode_into(&self) -> Vec<u8> {
        let mut v = self.codec.encode_into();
        v.append(&mut self.t.encode_into());
        v
    }
}

impl<T> TryFrom<&[u8]> for Tagged<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a>,
    for<'a> Error: From<<T as TryDecodeFrom<'a>>::Error>,
{
    type Error = Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        let (s, _) = Self::try_decode_from(v)?;
        Ok(s)
    }
}

impl<'a, T> TryDecodeFrom<'a> for Tagged<T>
where
    T: EncodeInto + for<'b> TryDecodeFrom<'b>,
    for<'b> Error: From<<T as TryDecodeFrom<'b>>::Error>,
{
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        let (codec, ptr) = Codec::try_decode_from(bytes)?;
        let (t, ptr) = T::try_decode_from(ptr)?;
        Ok((Self { codec, t }, ptr))
    }
}

impl<T> PartialEq for Tagged<T>
where
    T: EncodeInto + PartialEq<T> + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.codec == other.codec && self.t == other.t
    }
}

impl<T> ops::Deref for Tagged<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.t
    }
}

impl<T> fmt::Debug for Tagged<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (0x{:x})", self.codec.as_str(), self.codec.code())
    }
}
