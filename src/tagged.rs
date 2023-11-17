use crate::prelude::{Base, Codec, CodecInfo, DefaultEncoding, EncodeInto, Error, TryDecodeFrom};
use core::{fmt, ops};

/// Smart pointer for multicodec tagged data
#[derive(PartialEq)]
pub struct Tagged<T>(T)
where
    T: CodecInfo + DefaultEncoding + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized;

impl<T> Tagged<T>
where
    T: CodecInfo + DefaultEncoding + EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    /// Construct a Tagged smart pointer with the given multicodec codec
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T> CodecInfo for Tagged<T>
where
    T: CodecInfo + DefaultEncoding + EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    fn codec() -> Codec {
        T::codec()
    }
}

impl<T> DefaultEncoding for Tagged<T>
where
    T: CodecInfo + DefaultEncoding + EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    fn encoding() -> Base {
        T::encoding()
    }
}

impl<T> EncodeInto for Tagged<T>
where
    T: CodecInfo + DefaultEncoding + EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    fn encode_into(&self) -> Vec<u8> {
        let mut v = T::codec().encode_into();
        v.append(&mut self.0.encode_into());
        v
    }
}

impl<T> TryFrom<&[u8]> for Tagged<T>
where
    T: CodecInfo + DefaultEncoding + EncodeInto + for<'a> TryDecodeFrom<'a>,
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
    T: CodecInfo + DefaultEncoding + EncodeInto + for<'b> TryDecodeFrom<'b>,
    for<'b> Error: From<<T as TryDecodeFrom<'b>>::Error>,
{
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        let (codec, ptr) = Codec::try_decode_from(bytes)?;
        if codec != T::codec() {
            return Err(Error::IncorrectSigil {
                expected: T::codec(),
                received: codec,
            });
        }
        let (t, ptr) = T::try_decode_from(ptr)?;
        Ok((Self(t), ptr))
    }
}

impl<T> ops::Deref for Tagged<T>
where
    T: CodecInfo + DefaultEncoding + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> fmt::Debug for Tagged<T>
where
    T: CodecInfo + DefaultEncoding + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (0x{:x})", T::codec().as_str(), T::codec().code())
    }
}
