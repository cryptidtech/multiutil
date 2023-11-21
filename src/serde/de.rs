use crate::prelude::{BaseEncoded, CodecInfo, EncodeInto, EncodingInfo, Tagged, TryDecodeFrom};
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
        let (codec, t) = de::Deserialize::deserialize(deserializer)?;
        Ok(Self { codec, t })
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
        if deserializer.is_human_readable() {
            let s: String = de::Deserialize::deserialize(deserializer)?;
            match multibase::decode(s.as_str()) {
                Ok((base, v)) => {
                    let (t, _) = T::try_decode_from(v.as_slice())
                        .map_err(|_| de::Error::custom("failed".to_string()))?;
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
