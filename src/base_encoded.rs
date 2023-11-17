use crate::prelude::{base_name, Base, EncodeInto, Error, TryDecodeFrom};
use core::{fmt, ops};

/// Smart pointer for multibase encoded data
pub struct BaseEncoded<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    /// The multibase encoding codec
    pub base: Base,
    t: T,
}

impl<T> BaseEncoded<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    /// Construct a new BaseEncoded instance with the given base encoding codec
    pub fn new(base: Base, t: T) -> Self {
        Self { base, t }
    }
}

impl<T> EncodeInto for BaseEncoded<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    fn encode_into(&self) -> Vec<u8> {
        self.t.encode_into()
    }
}

impl<T> TryFrom<&str> for BaseEncoded<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a>,
    for<'a> Error: From<<T as TryDecodeFrom<'a>>::Error>,
{
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match multibase::decode(s) {
            Ok((base, v)) => {
                let (t, _) = T::try_decode_from(v.as_slice())?;
                Ok(Self { base, t })
            }
            Err(e) => Err(Error::Multibase(e)),
        }
    }
}

impl<T> PartialEq for BaseEncoded<T>
where
    T: EncodeInto + PartialEq<T> + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.t == other.t
    }
}

impl<T> ops::Deref for BaseEncoded<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.t
    }
}

impl<T> fmt::Display for BaseEncoded<T>
where
    T: EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", multibase::encode(self.base, &self.encode_into()))
    }
}

impl<T> fmt::Debug for BaseEncoded<T>
where
    T: fmt::Debug + EncodeInto + for<'a> TryDecodeFrom<'a>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ('{}') - {:?} - {}",
            base_name(self.base),
            self.base.code(),
            self.t,
            multibase::encode(self.base, &self.encode_into())
        )
    }
}
