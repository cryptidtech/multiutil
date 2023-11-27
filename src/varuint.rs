use core::fmt;
use multitrait::prelude::EncodeInto;

/// A wrapper type to handle serde of numeric types as varuint bytes
#[derive(Clone, PartialEq)]
pub struct Varuint<T>(pub(crate) T);

impl<T> Varuint<T> {
    /// consume self and return inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> fmt::Debug for Varuint<T>
where
    T: EncodeInto,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0.encode_into().as_slice())
    }
}
