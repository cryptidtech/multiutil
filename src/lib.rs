//!
#![warn(missing_docs)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

/// Errors generated from the implementations
pub mod error;

/// re-export the error;
pub use error::Error;

/// This trait tries to decode a type from a slice of bytes. This primarily
/// used for types encoded with varuint values.
pub trait TryDecodeFrom<'a>: Sized {
    /// The error type emited on failure
    type Error;

    /// Try to decode the type from a slice of bytes returning the object and
    /// the reference to the rest of the slice
    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error>;
}

use unsigned_varint::decode;

/// Try to decode a varuint encoded u8
impl<'a> TryDecodeFrom<'a> for u8 {
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        Ok(decode::u8(bytes).map_err(|e| Self::Error::UnsignedVarintDecode(e))?)
    }
}

/// Try to decode a varuint encoded u16
impl<'a> TryDecodeFrom<'a> for u16 {
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        Ok(decode::u16(bytes).map_err(|e| Self::Error::UnsignedVarintDecode(e))?)
    }
}

/// Try to decode a varuint encoded u32
impl<'a> TryDecodeFrom<'a> for u32 {
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        Ok(decode::u32(bytes).map_err(|e| Self::Error::UnsignedVarintDecode(e))?)
    }
}

/// Try to decode a varuint encoded u64
impl<'a> TryDecodeFrom<'a> for u64 {
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        Ok(decode::u64(bytes).map_err(|e| Self::Error::UnsignedVarintDecode(e))?)
    }
}

/// Try to decode a varuint encoded u128
impl<'a> TryDecodeFrom<'a> for u128 {
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        Ok(decode::u128(bytes).map_err(|e| Self::Error::UnsignedVarintDecode(e))?)
    }
}

/// Try to decode a varuint encoded u128
impl<'a> TryDecodeFrom<'a> for usize {
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        Ok(decode::usize(bytes).map_err(|e| Self::Error::UnsignedVarintDecode(e))?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use unsigned_varint::encode;

    #[test]
    fn test_u8() {
        let mut buf = encode::u8_buffer();
        let b = encode::u8(0xff_u8, &mut buf);
        let (num, _) = u8::try_decode_from(b).unwrap();
        assert_eq!(0xff_u8, num);
    }

    #[test]
    fn test_u16() {
        let mut buf = encode::u16_buffer();
        let b = encode::u16(0xffee_u16, &mut buf);
        let (num, _) = u16::try_decode_from(b).unwrap();
        assert_eq!(0xffee_u16, num);
    }

    #[test]
    fn test_u32() {
        let mut buf = encode::u32_buffer();
        let b = encode::u32(0xffeeddcc_u32, &mut buf);
        let (num, _) = u32::try_decode_from(b).unwrap();
        assert_eq!(0xffeeddcc_u32, num);
    }

    #[test]
    fn test_u64() {
        let mut buf = encode::u64_buffer();
        let b = encode::u64(0xffeeddcc_bbaa9988_u64, &mut buf);
        let (num, _) = u64::try_decode_from(b).unwrap();
        assert_eq!(0xffeeddcc_bbaa9988_u64, num);
    }

    #[test]
    fn test_u128() {
        let mut buf = encode::u128_buffer();
        let b = encode::u128(0xffeeddcc_bbaa9988_77665544_33221100_u128, &mut buf);
        let (num, _) = u128::try_decode_from(b).unwrap();
        assert_eq!(0xffeeddcc_bbaa9988_77665544_33221100_u128, num);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_usize() {
        let mut buf = encode::usize_buffer();
        let b = encode::usize(0xffeeddcc_bbaa9988_usize, &mut buf);
        let (num, _) = usize::try_decode_from(b).unwrap();
        assert_eq!(0xffeeddcc_bbaa9988_usize, num);
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_usize() {
        let mut buf = encode::usize_buffer();
        let b = encode::usize(0xffeeddcc_usize, &mut buf);
        let (num, _) = usize::try_decode_from(b).unwrap();
        assert_eq!(0xffeeddcc_usize, num);
    }
}
