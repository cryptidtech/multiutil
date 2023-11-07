pub trait TryDecodeFrom<'a>: Sized {
    type Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error>;
}
