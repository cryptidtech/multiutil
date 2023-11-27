//!
#![warn(missing_docs)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

/// BaseEncoded smart pointer
pub mod base_encoded;
pub use base_encoded::BaseEncoded;

/// base_name function
pub mod base_name;
pub use base_name::base_name;

/// CodecInfo trait
pub mod codec_info;
pub use codec_info::CodecInfo;

/// EncodingInfo trait
pub mod encoding_info;
pub use encoding_info::EncodingInfo;

/// Errors generated from the implementations
pub mod error;
pub use error::Error;

/// Serde serialization
#[cfg(feature = "serde")]
pub mod serde;

/// Varunit type for handling serde of numeric types
pub mod varuint;
pub use varuint::Varuint;

/// one-stop shop for all exported symbols
pub mod prelude {
    pub use super::{
        base_encoded::*, base_name::*, codec_info::*, encoding_info::*, error::*, varuint::*,
    };

    /// re-exports
    pub use multibase::Base;
    pub use multicodec::Codec;
}

#[cfg(test)]
mod test {
    use super::prelude::*;

    #[test]
    fn test_base_name() {
        assert_eq!(base_name(Base::Base16Upper), "Base16Upper".to_string());
    }

    #[derive(Clone, Debug, PartialEq)]
    struct Unit([u8; 2]);
    type EncodedUnit = BaseEncoded<Unit>;

    impl Unit {
        pub fn encoded_default() -> EncodedUnit {
            EncodedUnit::new(Self::default())
        }

        pub fn value(&self) -> u8 {
            self.0[0]
        }
    }

    impl Default for Unit {
        fn default() -> Self {
            Self([0x42, 0xAA])
        }
    }

    impl EncodingInfo for Unit {
        fn preferred_encoding() -> Base {
            Base::Base16Lower
        }

        fn encoding(&self) -> Base {
            Base::Base16Lower
        }
    }

    impl AsRef<[u8]> for Unit {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    impl<'a> TryFrom<&'a [u8]> for Unit {
        type Error = Error;

        fn try_from(s: &'a [u8]) -> Result<Self, Error> {
            if s.len() < 2 {
                Err(Error::custom("too few items in the vec"))
            } else {
                Ok(Self([s[0], s[1]]))
            }
        }
    }

    impl Into<Vec<u8>> for Unit {
        fn into(self) -> Vec<u8> {
            let mut v = Vec::default();
            v.extend_from_slice(&self.0);
            v
        }
    }

    #[test]
    fn test_display() {
        let betu = Unit::encoded_default();
        assert_eq!("f42aa".to_string(), betu.to_string());
    }

    #[test]
    fn test_try_from_str() {
        let betu = EncodedUnit::try_from("f42aa").unwrap();
        assert_eq!(Unit::encoded_default(), betu);
    }

    #[test]
    fn test_string_round_trip() {
        let betu1 = Unit::encoded_default();
        let s = betu1.to_string();
        let betu2 = EncodedUnit::try_from(s.as_str()).unwrap();
        assert_eq!(betu1, betu2);
    }

    #[test]
    fn test_bytes_round_trip() {
        let betu1 = Unit::encoded_default();
        let s = betu1.to_string();
        let betu2 = EncodedUnit::try_from(s.as_str()).unwrap();
        assert_eq!(betu1, betu2);
    }

    #[test]
    fn test_smart_pointer() {
        let betu = Unit::encoded_default();
        assert_eq!(betu.value(), 0x42);
    }

    #[test]
    fn test_as_ref() {
        let betu = Unit::encoded_default();
        assert_eq!(&[0x42, 0xAA], betu.to_inner().as_ref());
    }
}
