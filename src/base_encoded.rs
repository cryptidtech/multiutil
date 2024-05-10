// SPDX-License-Idnetifier: Apache-2.0
use crate::{
    error::BaseEncodedError, prelude::Base, BaseEncoder, EncodingInfo, Error, MultibaseEncoder,
};
use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops,
};

/// Smart pointer for multibase encoded data. This supports encoding to and
/// decoding from multibase encoding strings using [`TryFrom<&str>`] and
/// ['to_string()']
#[derive(Clone)]
pub struct BaseEncoded<T, Enc = MultibaseEncoder>
where
    T: EncodingInfo + ?Sized,
    Enc: BaseEncoder,
{
    pub(crate) enc: PhantomData<Enc>,
    pub(crate) base: Base,
    pub(crate) t: T,
}

impl<T, Enc> BaseEncoded<T, Enc>
where
    T: EncodingInfo,
    Enc: BaseEncoder,
{
    /// Construct a new BaseEncoded instance with the given base
    pub fn new(base: Base, t: T) -> Self {
        Self {
            base,
            t,
            enc: PhantomData,
        }
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
        Self::new(T::preferred_encoding(), t)
    }
}

impl<T, Enc> TryFrom<&str> for BaseEncoded<T, Enc>
where
    T: EncodingInfo + for<'a> TryFrom<&'a [u8]>,
    Enc: BaseEncoder,
{
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match Enc::from_base_encoded(s) {
            Ok((base, v)) => Ok(Self {
                base,
                t: T::try_from(v.as_slice()).map_err(|_| BaseEncodedError::ValueFailed)?,
                enc: PhantomData,
            }),
            Err(e) => Err(e.into()),
        }
    }
}

impl<T, Enc> PartialEq for BaseEncoded<T, Enc>
where
    T: EncodingInfo + PartialEq<T> + ?Sized,
    Enc: BaseEncoder,
{
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.t == other.t
    }
}

impl<T, Enc> PartialOrd for BaseEncoded<T, Enc>
where
    T: EncodingInfo + PartialEq<T> + PartialOrd<T> + ?Sized,
    Enc: BaseEncoder,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl<T, Enc> Eq for BaseEncoded<T, Enc>
where
    T: EncodingInfo + Eq + ?Sized,
    Enc: BaseEncoder,
{
}

impl<T, Enc> Ord for BaseEncoded<T, Enc>
where
    T: EncodingInfo + Eq + PartialOrd<T> + Ord + ?Sized,
    Enc: BaseEncoder,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.cmp(&other.t)
    }
}

impl<T> Hash for BaseEncoded<T>
where
    T: EncodingInfo + Clone + Into<Vec<u8>>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let s = self.to_string();
        s.hash(state);
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

impl<T, Enc> fmt::Display for BaseEncoded<T, Enc>
where
    T: EncodingInfo + Clone + Into<Vec<u8>>,
    Enc: BaseEncoder,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            Enc::to_base_encoded(self.base, &self.t.clone().into())
        )
    }
}

impl<T, Enc> fmt::Debug for BaseEncoded<T, Enc>
where
    T: fmt::Debug + EncodingInfo + Clone + Into<Vec<u8>>,
    Enc: BaseEncoder,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {:?}", Enc::debug_string(self.base), self.t,)
    }
}
