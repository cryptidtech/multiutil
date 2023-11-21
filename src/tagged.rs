use crate::prelude::{
    Base, Codec, CodecInfo, EncodeInto, EncodingInfo, TaggedError, TryDecodeFrom,
};
use core::{fmt, ops};

/// Smart pointer for multicodec tagged data
#[derive(PartialEq)]
pub struct Tagged<T>
where
    T: CodecInfo + EncodingInfo + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    pub(crate) codec: Codec,
    pub(crate) t: T,
}

impl<T> Tagged<T>
where
    T: CodecInfo + EncodingInfo + EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    /// Construct a Tagged smart pointer with the given multicodec codec
    pub fn new(t: T) -> Self {
        Self {
            codec: T::preferred_codec(),
            t,
        }
    }

    /// Construct a Tagged smart pointer with the given codec
    pub fn new_codec(codec: Codec, t: T) -> Self {
        Self { codec, t }
    }

    /// Convert to the inner T type, consuming self
    pub fn to_inner(self) -> T {
        self.t
    }
}

impl<T> CodecInfo for Tagged<T>
where
    T: CodecInfo + EncodingInfo + EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    /// Return the codec hint for the contained type
    fn preferred_codec() -> Codec {
        T::preferred_codec()
    }

    /// Return the codec for the contained object
    fn codec(&self) -> Codec {
        self.codec
    }
}

impl<T> EncodingInfo for Tagged<T>
where
    T: CodecInfo + EncodingInfo + EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    /// Return the encoding hint for the contained type
    fn preferred_encoding() -> Base {
        T::preferred_encoding()
    }

    /// Return the encoding hint again because string encoding wraps Tagged<T>
    fn encoding(&self) -> Base {
        Self::preferred_encoding()
    }
}

impl<T> Into<Vec<u8>> for Tagged<T>
where
    T: CodecInfo + EncodingInfo + EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    fn into(self) -> Vec<u8> {
        self.encode_into()
    }
}

impl<T> EncodeInto for Tagged<T>
where
    T: CodecInfo + EncodingInfo + EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    fn encode_into(&self) -> Vec<u8> {
        let mut v = self.codec().encode_into();
        v.append(&mut self.t.encode_into());
        v
    }
}

impl<T> TryFrom<&[u8]> for Tagged<T>
where
    T: CodecInfo + EncodingInfo + EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    type Error = TaggedError;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        let (s, _) = Self::try_decode_from(v)?;
        Ok(s)
    }
}

impl<'a, T> TryDecodeFrom<'a> for Tagged<T>
where
    T: CodecInfo + EncodingInfo + EncodeInto + for<'b> TryDecodeFrom<'b>,
{
    type Error = TaggedError;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        let (codec, ptr) = Codec::try_decode_from(bytes).map_err(|_| TaggedError::SigilFailed)?;
        if codec != T::preferred_codec() {
            return Err(TaggedError::IncorrectSigil {
                expected: T::preferred_codec(),
                received: codec,
            });
        }
        let (t, ptr) = T::try_decode_from(ptr).map_err(|_| TaggedError::ValueFailed)?;
        Ok((Self { codec, t }, ptr))
    }
}

impl<T> ops::Deref for Tagged<T>
where
    T: CodecInfo + EncodingInfo + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.t
    }
}

impl<T> fmt::Debug for Tagged<T>
where
    T: fmt::Debug + CodecInfo + EncodingInfo + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} (0x{:x}) - {:?}",
            self.codec().as_str(),
            self.codec().code(),
            self.t
        )
    }
}
