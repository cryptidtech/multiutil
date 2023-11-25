//! Serde (de)serialization for ['crate::prelude::Tagged'] wrapped objects
mod de;
mod ser;

#[cfg(test)]
mod tests {
    use crate::prelude::{Error, *};
    use serde::{de, ser};
    use serde_test::{assert_tokens, Configure, Token};
    //use std::fmt;

    #[derive(Clone, Debug, PartialEq)]
    struct Unit((u8, [u8; 2]));
    type EncodedUnit = BaseEncoded<Unit>;

    impl Unit {
        fn encoded_default() -> EncodedUnit {
            EncodedUnit::new(Unit::default())
        }
    }

    impl Default for Unit {
        fn default() -> Self {
            Self((0x59, [0xDE, 0xAD]))
        }
    }

    impl EncodingInfo for Unit {
        fn preferred_encoding() -> Base {
            Base::Base16Lower
        }

        fn encoding(&self) -> Base {
            Self::preferred_encoding()
        }
    }

    impl<'a> TryFrom<&'a [u8]> for Unit {
        type Error = Error;

        fn try_from(s: &'a [u8]) -> Result<Self, Error> {
            if s.len() < 3 {
                Err(Error::custom("too few items in the vec"))
            } else {
                Ok(Self((s[0], [s[1], s[2]])))
            }
        }
    }

    impl Into<Vec<u8>> for Unit {
        fn into(self) -> Vec<u8> {
            let mut v: Vec<u8> = Vec::default();
            v.push(self.0 .0);
            v.extend_from_slice(&self.0 .1);
            v
        }
    }

    impl ser::Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            (self.0 .0, self.0 .1).serialize(serializer)
        }
    }

    impl<'de> de::Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            let (v, arr) = de::Deserialize::deserialize(deserializer)?;
            Ok(Self((v, arr)))
        }
    }

    #[test]
    fn test_serde_base_encoded_readable() {
        let unit = Unit::encoded_default();
        assert_tokens(&unit.readable(), &[Token::String("f59dead")]);
    }

    #[test]
    fn test_serde_base_encoded_compact() {
        let unit = Unit::encoded_default();
        assert_tokens(
            &unit.compact(),
            &[
                Token::Tuple { len: 2 },
                Token::U8(0x59),
                Token::Tuple { len: 2 },
                Token::U8(0xDE),
                Token::U8(0xAD),
                Token::TupleEnd,
                Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn test_serde_json() {
        let unit = Unit::encoded_default();
        let unit_s = serde_json::to_string(&unit).unwrap();
        assert_eq!(unit_s, "\"f59dead\"".to_string());
    }

    #[test]
    fn test_serde_cbor() {
        let unit = Unit::encoded_default();
        let unit_cbor = serde_cbor::to_vec(&unit).unwrap();
        assert_eq!(unit_cbor, hex::decode("8218598218de18ad").unwrap());
    }
}
