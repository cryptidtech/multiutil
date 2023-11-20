use crate::prelude::{
    BaseEncoded, Codec, CodecInfo, EncodeInto, EncodingInfo, Tagged, TryDecodeFrom,
};
use core::{fmt, marker};
use serde::de;

/// Deserialize instance of [`crate::prelude::Tagged`] from a byte slice
impl<'de, T> de::Deserialize<'de> for Tagged<T>
where
    T: de::Deserialize<'de>
        + CodecInfo
        + EncodingInfo
        + EncodeInto
        + for<'a> TryDecodeFrom<'a>
        + ?Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct TaggedVisitor<T>(marker::PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for TaggedVisitor<T>
        where
            T: de::Deserialize<'de>
                + CodecInfo
                + EncodingInfo
                + EncodeInto
                + for<'a> TryDecodeFrom<'a>
                + ?Sized,
        {
            type Value = Tagged<T>;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(fmt, "tagged object byte slice")
            }

            #[inline]
            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: de::SeqAccess<'de>,
            {
                let codec: Codec = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(0, &self))?;
                if codec != T::preferred_codec() {
                    return Err(de::Error::custom(format!(
                        "Expected {:?} sigil but received {:?} sigil",
                        T::preferred_codec(),
                        codec
                    )));
                }
                let t: T = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(1, &self))?;
                Ok(Tagged::<T>::new(t))
            }
        }

        deserializer.deserialize_tuple(2, TaggedVisitor::<T>(marker::PhantomData::<T>))
    }
}

/// Deserialize instance of [`crate::prelude::BaseEncoded`] from a byte slice
impl<'de, T> de::Deserialize<'de> for BaseEncoded<T>
where
    T: de::Deserialize<'de>
        + CodecInfo
        + EncodingInfo
        + EncodeInto
        + for<'a> TryDecodeFrom<'a>
        + ?Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct BaseEncodedVisitor<T>(marker::PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for BaseEncodedVisitor<T>
        where
            T: de::Deserialize<'de>
                + CodecInfo
                + EncodingInfo
                + EncodeInto
                + for<'a> TryDecodeFrom<'a>
                + ?Sized,
        {
            type Value = BaseEncoded<T>;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(fmt, "tagged object byte slice")
            }

            #[inline]
            fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match multibase::decode(s.as_str()) {
                    Ok((base, v)) => {
                        let (t, _) = T::try_decode_from(v.as_slice())
                            .map_err(|_| de::Error::custom("failed".to_string()))?;
                        Ok(BaseEncoded::new_base(base, t))
                    }
                    Err(e) => Err(de::Error::custom(e.to_string())),
                }
            }
        }

        deserializer.deserialize_str(BaseEncodedVisitor::<T>(marker::PhantomData::<T>))
    }
}
