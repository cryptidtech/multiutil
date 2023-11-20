use crate::prelude::{BaseEncoded, CodecInfo, DefaultEncoding, EncodeInto, Tagged, TryDecodeFrom};
use core::ops::Deref;
use serde::ser::{self, SerializeTuple};

/// Serialize instance of [`crate::prelude::Tagged`] into varuint encoded bytes
impl<T> ser::Serialize for Tagged<T>
where
    T: ser::Serialize
        + CodecInfo
        + DefaultEncoding
        + EncodeInto
        + for<'a> TryDecodeFrom<'a>
        + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut t = serializer.serialize_tuple(2)?;
        t.serialize_element(&T::codec())?;
        t.serialize_element(self.deref())?;
        t.end()
    }
}

/// Serialize instance of [`crate::prelude::BaseEncoded`] into a string
impl<T> ser::Serialize for BaseEncoded<T>
where
    T: ser::Serialize
        + CodecInfo
        + DefaultEncoding
        + EncodeInto
        + for<'a> TryDecodeFrom<'a>
        + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Error, *};
    use serde_test::{assert_ser_tokens, Token};

    #[derive(PartialEq)]
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
    impl ser::Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_bytes(&self.0)
        }
    }

    #[test]
    fn test_ser_tagged() {
        let tagged = Tagged::new(Unit([0xDE, 0xAD]));
        assert_ser_tokens(
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
    fn test_ser_base_encoded() {
        let benc = BaseEncoded::new(Unit([0xDE, 0xAD]));
        assert_ser_tokens(&benc, &[Token::String("fdead")]);
    }
}
