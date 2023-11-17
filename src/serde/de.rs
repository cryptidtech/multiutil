use crate::prelude::{BaseEncoded, EncodeInto, Tagged, TryDecodeFrom};
use core::{fmt, marker};
use serde::de;

/// Deserialize instance of [`crate::prelude::Tagged`] from a byte slice
impl<'de, T> de::Deserialize<'de> for Tagged<T>
where
    T: de::Deserialize<'de> + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct TaggedVisitor<T>(marker::PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for TaggedVisitor<T>
        where
            T: de::Deserialize<'de> + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
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
                let codec = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(0, &self))?;
                let t = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(1, &self))?;
                Ok(Tagged::<T>::new(codec, t))
            }
        }

        deserializer.deserialize_tuple(2, TaggedVisitor::<T>(marker::PhantomData::<T>))
    }
}

/// Deserialize instance of [`crate::prelude::BaseEncoded`] from a byte slice
impl<'de, T> de::Deserialize<'de> for BaseEncoded<T>
where
    T: de::Deserialize<'de> + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct BaseEncodedVisitor<T>(marker::PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for BaseEncodedVisitor<T>
        where
            T: de::Deserialize<'de> + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
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
                        Ok(BaseEncoded::new(base, t))
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
    use crate::prelude::{Base, BaseEncoded, Codec, Tagged};
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn test_de_tagged() {
        let tagged = Tagged::new(Codec::Ed25519Pub, 0xAAu8);
        assert_de_tokens(
            &tagged,
            &[
                Token::Tuple { len: 2 },
                Token::U64(0xED),
                Token::U8(0xAA),
                Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn test_de_base_encoded() {
        let benc = BaseEncoded::new(Base::Base16Lower, 0xEDu8);
        assert_de_tokens(&benc, &[Token::String("fed01")]);
    }
}
