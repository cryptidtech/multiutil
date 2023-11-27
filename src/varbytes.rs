/// A wrapper type to handle serde of byte arrays as bytes
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Varbytes(pub Vec<u8>);

impl Varbytes {
    /// consume self and return inner vec
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}
