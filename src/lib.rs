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

/// This trait encodes a numeric value into a compact varuint Vec<u8>
pub trait EncodeInto {
    /// encode the type into a compact varuint Vec<u8>
    fn encode_into(self) -> Vec<u8>;
}

use unsigned_varint::{decode, encode};

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

/// Encode a u8 into a compact varuint Vec<u8>
impl EncodeInto for u8 {
    fn encode_into(self) -> Vec<u8> {
        let mut buf = encode::u8_buffer();
        encode::u8(self, &mut buf);
        let mut v: Vec<u8> = Vec::new();
        for b in buf {
            v.push(b);
            if decode::is_last(b) {
                break;
            }
        }
        v
    }
}

/// Encode a u16 into a compact varuint Vec<u8>
impl EncodeInto for u16 {
    fn encode_into(self) -> Vec<u8> {
        let mut buf = encode::u16_buffer();
        encode::u16(self, &mut buf);
        let mut v: Vec<u8> = Vec::new();
        for b in buf {
            v.push(b);
            if decode::is_last(b) {
                break;
            }
        }
        v
    }
}

/// Encode a u32 into a compact varuint Vec<u8>
impl EncodeInto for u32 {
    fn encode_into(self) -> Vec<u8> {
        let mut buf = encode::u32_buffer();
        encode::u32(self, &mut buf);
        let mut v: Vec<u8> = Vec::new();
        for b in buf {
            v.push(b);
            if decode::is_last(b) {
                break;
            }
        }
        v
    }
}

/// Encode a u64 into a compact varuint Vec<u8>
impl EncodeInto for u64 {
    fn encode_into(self) -> Vec<u8> {
        let mut buf = encode::u64_buffer();
        encode::u64(self, &mut buf);
        let mut v: Vec<u8> = Vec::new();
        for b in buf {
            v.push(b);
            if decode::is_last(b) {
                break;
            }
        }
        v
    }
}

/// Encode a u128 into a compact varuint Vec<u8>
impl EncodeInto for u128 {
    fn encode_into(self) -> Vec<u8> {
        let mut buf = encode::u128_buffer();
        encode::u128(self, &mut buf);
        let mut v: Vec<u8> = Vec::new();
        for b in buf {
            v.push(b);
            if decode::is_last(b) {
                break;
            }
        }
        v
    }
}

/// Encode a usize into a compact varuint Vec<u8>
impl EncodeInto for usize {
    fn encode_into(self) -> Vec<u8> {
        let mut buf = encode::usize_buffer();
        encode::usize(self, &mut buf);
        let mut v: Vec<u8> = Vec::new();
        for b in buf {
            v.push(b);
            if decode::is_last(b) {
                break;
            }
        }
        v
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u8() {
        let buf = 0xff_u8.encode_into();
        let (num, _) = u8::try_decode_from(&buf).unwrap();
        assert_eq!(0xff_u8, num);
    }

    #[test]
    fn test_u16() {
        let buf = 0xffee_u16.encode_into();
        let (num, _) = u16::try_decode_from(&buf).unwrap();
        assert_eq!(0xffee_u16, num);
    }

    #[test]
    fn test_u32() {
        let buf = 0xffeeddcc_u32.encode_into();
        let (num, _) = u32::try_decode_from(&buf).unwrap();
        assert_eq!(0xffeeddcc_u32, num);
    }

    #[test]
    fn test_u64() {
        let buf = 0xffeeddcc_bbaa9988_u64.encode_into();
        let (num, _) = u64::try_decode_from(&buf).unwrap();
        assert_eq!(0xffeeddcc_bbaa9988_u64, num);
    }

    #[test]
    fn test_u128() {
        let buf = 0xffeeddcc_bbaa9988_77665544_33221100_u128.encode_into();
        let (num, _) = u128::try_decode_from(&buf).unwrap();
        assert_eq!(0xffeeddcc_bbaa9988_77665544_33221100_u128, num);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_usize() {
        let buf = 0xffeeddcc_bbaa9988_usize.encode_into();
        let (num, _) = usize::try_decode_from(&buf).unwrap();
        assert_eq!(0xffeeddcc_bbaa9988_usize, num);
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_usize() {
        let buf = 0xffeeddcc_usize.encode_into();
        let (num, _) = usize::try_decode_from(&buf).unwrap();
        assert_eq!(0xffeeddcc_usize, num);
    }
}
