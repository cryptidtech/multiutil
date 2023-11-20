use crate::prelude::{
    base_name, Base, BaseEncodedError, CodecInfo, DefaultEncoding, EncodeInto, TryDecodeFrom,
};
use core::{fmt, ops};

/// Smart pointer for multibase encoded data. This supports encoding to and
/// decoding from multibase encoding strings using [`TryFrom<&str>`] and
///
pub struct BaseEncoded<T>
where
    T: CodecInfo + DefaultEncoding + ?Sized,
{
    /// The multibase encoding codec
    pub base: Base,
    t: T,
}

impl<T> BaseEncoded<T>
where
    T: CodecInfo + DefaultEncoding,
{
    /// Construct a new BaseEncoded instance using the default base encoding
    /// from the inner type
    pub fn new(t: T) -> Self {
        Self {
            base: T::encoding(),
            t,
        }
    }

    /// Construct a new BaseEncoded instance with the given base
    pub fn new_base(base: Base, t: T) -> Self {
        Self { base, t }
    }

    /// Convert to the inner T type, consuming self
    pub fn to_inner(self) -> T {
        self.t
    }
}

impl<T> TryFrom<&str> for BaseEncoded<T>
where
    T: CodecInfo + DefaultEncoding + for<'a> TryFrom<&'a [u8]>,
{
    type Error = BaseEncodedError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match multibase::decode(s) {
            Ok((base, v)) => Ok(Self {
                base,
                t: T::try_from(v.as_slice()).map_err(|_| BaseEncodedError::ValueFailed)?,
            }),
            Err(e) => Err(BaseEncodedError::Multibase(e)),
        }
    }
}

impl<'a, T> TryDecodeFrom<'a> for BaseEncoded<T>
where
    T: CodecInfo + DefaultEncoding + EncodeInto + for<'b> TryDecodeFrom<'b>,
{
    type Error = BaseEncodedError;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        let (t, ptr) = T::try_decode_from(bytes).map_err(|_| BaseEncodedError::ValueFailed)?;
        Ok((
            Self {
                base: T::encoding(),
                t,
            },
            ptr,
        ))
    }
}

impl<T> PartialEq for BaseEncoded<T>
where
    T: CodecInfo + DefaultEncoding + PartialEq<T> + ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.t == other.t
    }
}

impl<T> ops::Deref for BaseEncoded<T>
where
    T: CodecInfo + DefaultEncoding,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.t
    }
}

impl<T> fmt::Display for BaseEncoded<T>
where
    T: CodecInfo + DefaultEncoding + EncodeInto,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", multibase::encode(self.base, &self.t.encode_into()))
    }
}

impl<T> fmt::Debug for BaseEncoded<T>
where
    T: fmt::Debug + CodecInfo + DefaultEncoding + EncodeInto,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ('{}') - {:?} - {}",
            base_name(self.base),
            self.base.code(),
            self.t,
            multibase::encode(self.base, &self.t.encode_into())
        )
    }
}
