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

/// base_name function
pub mod base_name;

/// CodecInfo trait
pub mod codec_info;

/// DefaultEncoding trait
pub mod default_encoding;

/// Errors generated from the implementations
pub mod error;
pub use error::Error;

/// Serde serialization
#[cfg(feature = "serde")]
pub mod serde;

/// Tagged smart pointer
pub mod tagged;

/// one-stop shop for all exported symbols
pub mod prelude {
    pub use super::{
        base_encoded::*, base_name::*, codec_info::*, default_encoding::*, error::*, tagged::*,
    };

    /// re-exports
    pub use multibase::Base;
    pub use multicodec::prelude::Codec;
    pub use multitrait::prelude::{EncodeInto, TryDecodeFrom};
}

#[cfg(test)]
mod test {
    use super::{prelude::*, Error};

    #[test]
    fn test_base_name() {
        assert_eq!(base_name(Base::Base16Upper), "Base16Upper".to_string());
    }

    #[derive(Debug, PartialEq)]
    struct Unit([u8; 2]);
    impl Unit {
        pub fn value(&self) -> u8 {
            self.0[0]
        }
    }
    impl Default for Unit {
        fn default() -> Self {
            Self([0x42, 0xAA])
        }
    }
    impl AsRef<[u8]> for Unit {
        fn as_ref(&self) -> &[u8] {
            &self.0[..]
        }
    }
    impl EncodeInto for Unit {
        fn encode_into(&self) -> Vec<u8> {
            self.0.to_vec()
        }
    }
    impl<'a> TryDecodeFrom<'a> for Unit {
        type Error = Error;

        fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
            Ok((Self(bytes[..2].try_into().unwrap()), &bytes[2..]))
        }
    }
    impl CodecInfo for Unit {
        fn codec() -> Codec {
            Codec::Multihash
        }
    }
    impl DefaultEncoding for Unit {
        fn encoding() -> Base {
            Base::Base16Lower
        }
    }

    type BaseTagged = BaseEncoded<Tagged<Unit>>;

    impl Default for BaseTagged {
        fn default() -> Self {
            BaseEncoded::new(Tagged::new(Unit::default()))
        }
    }

    #[test]
    fn test_display() {
        let betu = BaseTagged::default();
        assert_eq!("f3142aa".to_string(), betu.to_string());
    }

    #[test]
    fn test_try_from_str() {
        let betu = BaseTagged::try_from("f3142aa").unwrap();
        assert_eq!(BaseTagged::default(), betu);
    }

    #[test]
    fn test_string_round_trip() {
        let betu1 = BaseTagged::default();
        let s = betu1.to_string();
        let betu2 = BaseTagged::try_from(s.as_str()).unwrap();
        assert_eq!(betu1, betu2);
    }

    #[test]
    fn test_bytes_round_trip() {
        let betu1 = BaseTagged::default();
        let s = betu1.encode_into();
        let (betu2, _) = BaseTagged::try_decode_from(s.as_slice()).unwrap();
        assert_eq!(betu1, betu2);
    }

    #[test]
    fn test_smart_pointer() {
        let betu = BaseTagged::default();
        assert_eq!(betu.value(), 0x42);
    }

    #[test]
    fn test_as_ref() {
        let betu = BaseTagged::default();
        assert_eq!(&[0x42, 0xAA], betu.as_ref());
    }
}
