use crate::prelude::{BaseEncoded, EncodingInfo, Varuint};
use multitrait::prelude::EncodeInto;
use serde::ser;

/// Serialize instance of [`crate::prelude::BaseEncoded`] into
impl<T> ser::Serialize for BaseEncoded<T>
where
    T: ser::Serialize + EncodingInfo + Clone + Into<Vec<u8>> + ?Sized,
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

/// Serialize instance of [`crate::prelude::Varuint`]
impl<T> ser::Serialize for Varuint<T>
where
    T: EncodeInto,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_bytes(self.0.encode_into().as_slice())
    }
}
