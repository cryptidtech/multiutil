use crate::prelude::{base_name, Base, BaseEncodedError, EncodingInfo};
use core::{fmt, ops};

/// Smart pointer for multibase encoded data. This supports encoding to and
/// decoding from multibase encoding strings using [`TryFrom<&str>`] and
///
#[derive(Clone)]
pub struct BaseEncoded<T>
where
    T: EncodingInfo + ?Sized,
{
    pub(crate) base: Base,
    pub(crate) t: T,
}

impl<T> BaseEncoded<T>
where
    T: EncodingInfo,
{
    /// Construct a new BaseEncoded instance using the default base encoding
    /// from the inner type
    pub fn new(t: T) -> Self {
        Self {
            base: T::preferred_encoding(),
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

impl<T> EncodingInfo for BaseEncoded<T>
where
    T: EncodingInfo,
{
    /// Return the encoding hint for the contained type
    fn preferred_encoding() -> Base {
        T::preferred_encoding()
    }

    /// Return the encoding used to encode the contained object
    fn encoding(&self) -> Base {
        self.base
    }
}

impl<T> From<T> for BaseEncoded<T>
where
    T: EncodingInfo,
{
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T> TryFrom<&str> for BaseEncoded<T>
where
    T: EncodingInfo + for<'a> TryFrom<&'a [u8]>,
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

impl<T> PartialEq for BaseEncoded<T>
where
    T: EncodingInfo + PartialEq<T> + ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.t == other.t
    }
}

impl<T> ops::Deref for BaseEncoded<T>
where
    T: EncodingInfo,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.t
    }
}

impl<T> ops::DerefMut for BaseEncoded<T>
where
    T: EncodingInfo,
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

impl<T> fmt::Display for BaseEncoded<T>
where
    T: EncodingInfo + Clone + Into<Vec<u8>>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            multibase::encode(self.base, &self.t.clone().into())
        )
    }
}

impl<T> fmt::Debug for BaseEncoded<T>
where
    T: fmt::Debug + EncodingInfo + Clone + Into<Vec<u8>>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ('{}') - {:?} - {}",
            base_name(self.base),
            self.base.code(),
            self.t,
            multibase::encode(self.base, &self.t.clone().into())
        )
    }
}
