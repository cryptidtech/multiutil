use crate::prelude::{BaseEncoded, CodecInfo, EncodeInto, EncodingInfo, Tagged, TryDecodeFrom};
use core::ops::Deref;
use serde::ser::{self, SerializeTuple};

/// Serialize instance of [`crate::prelude::Tagged`] into varuint encoded bytes
impl<T> ser::Serialize for Tagged<T>
where
    T: ser::Serialize + CodecInfo + EncodingInfo + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut t = serializer.serialize_tuple(2)?;
        t.serialize_element(&self.codec())?;
        t.serialize_element(self.deref())?;
        t.end()
    }
}

/// Serialize instance of [`crate::prelude::BaseEncoded`] into a string
impl<T> ser::Serialize for BaseEncoded<T>
where
    T: ser::Serialize + CodecInfo + EncodingInfo + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
