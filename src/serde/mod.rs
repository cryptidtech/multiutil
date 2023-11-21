//! Serde (de)serialization for ['crate::prelude::Tagged'] wrapped objects
mod de;
mod ser;

#[cfg(test)]
mod tests {
    use crate::prelude::{Error, *};
    use serde::{de, ser};
    use serde_test::{assert_tokens, Configure, Token};

    #[derive(Debug, PartialEq)]
    struct UnitImpl((u8, [u8; 2]));
    impl Default for UnitImpl {
        fn default() -> Self {
            Self((0x59, [0x42, 0xAA]))
        }
    }
    impl AsRef<[u8]> for UnitImpl {
        fn as_ref(&self) -> &[u8] {
            &self.0 .1[..]
        }
    }
    impl EncodeInto for UnitImpl {
        fn encode_into(&self) -> Vec<u8> {
            let mut v = self.0 .0.encode_into();
            v.append(&mut self.0 .1.to_vec());
            v
        }
    }
    impl<'a> TryDecodeFrom<'a> for UnitImpl {
        type Error = Error;

        fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
            let (v, ptr) = u8::try_decode_from(bytes)?;
            let arr = ptr[..2].try_into().unwrap();
            Ok((Self((v, arr)), &ptr[2..]))
        }
    }
    impl CodecInfo for UnitImpl {
        fn preferred_codec() -> Codec {
            Codec::Ed25519Pub
        }

        fn codec(&self) -> Codec {
            Self::preferred_codec()
        }
    }
    impl EncodingInfo for UnitImpl {
        fn preferred_encoding() -> Base {
            Base::Base16Lower
        }

        fn encoding(&self) -> Base {
            Self::preferred_encoding()
        }
    }
    impl ser::Serialize for UnitImpl {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            (self.0 .0, self.0 .1).serialize(serializer)
        }
    }
    impl<'de> de::Deserialize<'de> for UnitImpl {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            let (v, arr) = de::Deserialize::deserialize(deserializer)?;
            Ok(Self((v, arr)))
        }
    }

    #[test]
    fn test_serde_tagged() {
        let tagged = Tagged::new(UnitImpl((0x59, [0xDE, 0xAD])));
        assert_tokens(
            &tagged,
            &[
                Token::Tuple { len: 2 },
                Token::U64(0xed),
                Token::Tuple { len: 2 },
                Token::U8(0x59),
                Token::Tuple { len: 2 },
                Token::U8(0xDE),
                Token::U8(0xAD),
                Token::TupleEnd,
                Token::TupleEnd,
                Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn test_serde_base_encoded_readable() {
        let unit = BaseEncoded::new(Tagged::new(UnitImpl((0x59, [0xDE, 0xAD]))));
        assert_tokens(&unit.readable(), &[Token::String("fed0159dead")]);
    }

    #[test]
    fn test_serde_base_encoded_compact() {
        let unit = BaseEncoded::new(Tagged::new(UnitImpl((0x59, [0xDE, 0xAD]))));
        assert_tokens(
            &unit.compact(),
            &[
                Token::Tuple { len: 2 },
                Token::U64(0xed),
                Token::Tuple { len: 2 },
                Token::U8(0x59),
                Token::Tuple { len: 2 },
                Token::U8(0xDE),
                Token::U8(0xAD),
                Token::TupleEnd,
                Token::TupleEnd,
                Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn test_serde_json() {
        let unit = BaseEncoded::new(Tagged::new(UnitImpl((0x59, [0xDE, 0xAD]))));
        let unit_s = serde_json::to_string(&unit).unwrap();
        assert_eq!(unit_s, "\"fed0159dead\"".to_string());
    }

    #[test]
    fn test_serde_cbor() {
        let unit = BaseEncoded::new(Tagged::new(UnitImpl((0x59, [0xDE, 0xAD]))));
        let unit_cbor = serde_cbor::to_vec(&unit).unwrap();
        assert_eq!(unit_cbor, hex::decode("8218ed8218598218de18ad").unwrap());
    }
}
