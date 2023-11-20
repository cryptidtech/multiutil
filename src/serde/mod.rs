//! Serde (de)serialization for ['crate::prelude::Tagged'] wrapped objects
mod de;
mod ser;

#[cfg(test)]
mod tests {
    use crate::prelude::{Error, *};
    use core::fmt;
    use serde::{de, ser};
    use serde_test::{assert_tokens, Token};

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
        fn preferred_codec() -> Codec {
            Codec::Multihash
        }

        fn codec(&self) -> Codec {
            Codec::Multihash
        }
    }
    impl EncodingInfo for Unit {
        fn preferred_encoding() -> Base {
            Base::Base16Lower
        }

        fn encoding(&self) -> Base {
            Base::Base16Lower
        }
    }
    impl ser::Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_bytes(&self.0)
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
    fn test_serde_tagged() {
        let tagged = Tagged::new(Unit([0xDE, 0xAD]));
        assert_tokens(
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
    fn test_serde_base_encoded() {
        let benc = BaseEncoded::new(Unit([0xDE, 0xAD]));
        assert_tokens(&benc, &[Token::String("fdead")]);
    }
}
