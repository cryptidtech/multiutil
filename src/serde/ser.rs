use crate::prelude::{BaseEncoded, EncodingInfo};
use serde::ser;

/// Serialize instance of [`crate::prelude::BaseEncoded`] into a string
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
