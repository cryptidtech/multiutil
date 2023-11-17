use crate::prelude::{
    BaseEncoded, Codec, CodecInfo, DefaultEncoding, EncodeInto, Tagged, TryDecodeFrom,
};
use core::{fmt, marker};
use serde::de;

/// Deserialize instance of [`crate::prelude::Tagged`] from a byte slice
impl<'de, T> de::Deserialize<'de> for Tagged<T>
where
    T: de::Deserialize<'de> + CodecInfo + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct TaggedVisitor<T>(marker::PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for TaggedVisitor<T>
        where
            T: de::Deserialize<'de> + CodecInfo + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
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
                if codec != T::codec() {
                    return Err(de::Error::custom(format!(
                        "Expected {:?} sigil but received {:?} sigil",
                        T::codec(),
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
    T: de::Deserialize<'de> + DefaultEncoding + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct BaseEncodedVisitor<T>(marker::PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for BaseEncodedVisitor<T>
        where
            T: de::Deserialize<'de>
                + DefaultEncoding
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Error, *};
    use serde_test::{assert_de_tokens, Token};

    #[derive(Debug, PartialEq)]
    struct Unit([u8; 2]);
    impl Default for Unit {
        fn default() -> Self {
            Self([0x42, 0xAA])
        }
    }
    impl AsRef<[u8]> for Unit {
        fn as_ref(&self) -> &[u8] {
            &self.0[..]
        }
    }
    impl EncodeInto for Unit {
        fn encode_into(&self) -> Vec<u8> {
            self.0.to_vec()
        }
    }
    impl<'a> TryDecodeFrom<'a> for Unit {
        type Error = Error;

        fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
            Ok((Self(bytes[..2].try_into().unwrap()), &bytes[2..]))
        }
    }
    impl CodecInfo for Unit {
        fn codec() -> Codec {
            Codec::Multihash
        }
    }
    impl DefaultEncoding for Unit {
        fn encoding() -> Base {
            Base::Base16Lower
        }
    }
    impl<'de> de::Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct UnitVisitor;

            impl<'de> de::Visitor<'de> for UnitVisitor {
                type Value = Unit;

                fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(fmt, "two bytes")
                }

                fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Unit(v[..2].try_into().unwrap()))
                }
            }

            deserializer.deserialize_bytes(UnitVisitor)
        }
    }

    #[test]
    fn test_de_tagged() {
        let tagged = Tagged::new(Unit([0xDE, 0xAD]));
        assert_de_tokens(
            &tagged,
            &[
                Token::Tuple { len: 2 },
                Token::U64(0x31),
                Token::Bytes(&[0xDE, 0xAD]),
                Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn test_de_base_encoded() {
        let benc = BaseEncoded::new(Unit([0xDE, 0xAD]));
        assert_de_tokens(&benc, &[Token::String("fdead")]);
    }
}
