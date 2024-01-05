use crate::{BaseEncoded, BaseEncoder, EncodingInfo, Varbytes, Varuint};
use core::{fmt, marker};
use multibase::Base;
use multitrait::prelude::TryDecodeFrom;
use serde::de;

/// Deserialize instance of [`crate::BaseEncoded`] from a byte slice
impl<'de, T, Enc> de::Deserialize<'de> for BaseEncoded<T, Enc>
where
    T: de::Deserialize<'de> + EncodingInfo + for<'a> TryFrom<&'a [u8]> + ?Sized,
    Enc: BaseEncoder,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s: String = de::Deserialize::deserialize(deserializer)?;
            Self::try_from(s.as_str()).map_err(|e| de::Error::custom(e.to_string()))
        } else {
            let (base, t): (char, T) = de::Deserialize::deserialize(deserializer)?;
            Ok(Self {
                enc: marker::PhantomData,
                base: Base::from_code(base).map_err(|e| de::Error::custom(e.to_string()))?,
                t,
            })
        }
    }
}

/// Deserialize instance of [`crate::Varuint`] from a byte slice
impl<'de, T> de::Deserialize<'de> for Varuint<T>
where
    T: for<'a> TryDecodeFrom<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct VaruintVisitor<T>(marker::PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for VaruintVisitor<T>
        where
            T: for<'a> TryDecodeFrom<'a>,
        {
            type Value = Varuint<T>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "varuint encoded numeric value")
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let (t, _) = T::try_decode_from(v)
                    .map_err(|_| de::Error::custom("failed to deserialize varuint bytes"))?;
                Ok(Varuint(t))
            }
        }

        deserializer.deserialize_bytes(VaruintVisitor::<T>(marker::PhantomData::<T>))
    }
}

/// Deserialize instance of [`crate::Varbytes`] from a byte slice
impl<'de> de::Deserialize<'de> for Varbytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct VarbytesVisitor;

        impl<'de> de::Visitor<'de> for VarbytesVisitor {
            type Value = Varbytes;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "varuint encoded len followed by bytes")
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let (len, ptr) = usize::try_decode_from(v)
                    .map_err(|_| de::Error::custom("failed to deserialize varuint len"))?;
                let v = ptr[..len].to_vec();
                Ok(Varbytes(v))
            }
        }

        deserializer.deserialize_bytes(VarbytesVisitor)
    }
}
