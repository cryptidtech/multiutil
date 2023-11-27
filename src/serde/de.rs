use crate::prelude::{BaseEncoded, EncodingInfo, Varuint};
use core::{fmt, marker};
use multitrait::prelude::TryDecodeFrom;
use serde::de;

/// Deserialize instance of [`crate::prelude::BaseEncoded`] from a byte slice
impl<'de, T> de::Deserialize<'de> for BaseEncoded<T>
where
    T: de::Deserialize<'de> + EncodingInfo + for<'a> TryFrom<&'a [u8]> + ?Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s: String = de::Deserialize::deserialize(deserializer)?;
            match multibase::decode(s.as_str()) {
                Ok((base, v)) => {
                    let t = T::try_from(&v).map_err(|_| de::Error::custom("failed".to_string()))?;
                    Ok(Self { base, t })
                }
                Err(e) => Err(de::Error::custom(e.to_string())),
            }
        } else {
            let t: T = de::Deserialize::deserialize(deserializer)?;
            Ok(Self {
                base: T::preferred_encoding(),
                t,
            })
        }
    }
}

/// Deserialize instance of [`crate::prelude::Varuint`] from a byte slice
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
