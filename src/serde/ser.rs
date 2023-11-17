use crate::prelude::{BaseEncoded, EncodeInto, Tagged, TryDecodeFrom};
use core::ops::Deref;
use serde::ser::{self, SerializeTuple};

/// Serialize instance of [`crate::prelude::Tagged`] into varuint encoded bytes
impl<T> ser::Serialize for Tagged<T>
where
    T: ser::Serialize + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut t = serializer.serialize_tuple(2)?;
        t.serialize_element(&self.codec)?;
        t.serialize_element(self.deref())?;
        t.end()
    }
}

/// Serialize instance of [`crate::prelude::BaseEncoded`] into a string
impl<T> ser::Serialize for BaseEncoded<T>
where
    T: ser::Serialize + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Base, BaseEncoded, Codec, Tagged};
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn test_ser_tagged() {
        let tagged = Tagged::new(Codec::Ed25519Pub, 0xAAu8);
        assert_ser_tokens(
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
    fn test_ser_base_encoded() {
        let benc = BaseEncoded::new(Base::Base16Lower, 0xEDu8);
        assert_ser_tokens(&benc, &[Token::String("fed01")]);
    }
}
