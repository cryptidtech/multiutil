use crate::prelude::{BaseEncoded, CodecInfo, EncodeInto, EncodingInfo, Tagged, TryDecodeFrom};
use core::ops::Deref;
use serde::ser;

/// Serialize instance of [`crate::prelude::Tagged`] into varuint encoded bytes
impl<T> ser::Serialize for Tagged<T>
where
    T: ser::Serialize + CodecInfo + EncodingInfo + EncodeInto + for<'a> TryDecodeFrom<'a> + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (self.codec(), self.deref()).serialize(serializer)
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
        if serializer.is_human_readable() {
            self.to_string().as_str().serialize(serializer)
        } else {
            self.t.serialize(serializer)
        }
    }
}
