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
    pub use super::{base_encoded::*, base_name::*, error::*, tagged::*};

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

    #[derive(PartialEq)]
    struct Unit(u8);
    impl Default for Unit {
        fn default() -> Self {
            Self(0x42)
        }
    }
    impl EncodeInto for Unit {
        fn encode_into(&self) -> Vec<u8> {
            vec![self.0]
        }
    }
    impl<'a> TryDecodeFrom<'a> for Unit {
        type Error = Error;

        fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
            let (b, ptr) = u8::try_decode_from(bytes)?;
            Ok((Self(b), ptr))
        }
    }

    type BaseTagged = BaseEncoded<Tagged<Unit>>;

    impl Default for BaseTagged {
        fn default() -> Self {
            BaseEncoded::new(
                Base::Base16Lower,
                Tagged::new(Codec::Multihash, Unit::default()),
            )
        }
    }

    #[test]
    fn test_display() {
        let betu = BaseTagged::default();
        assert_eq!("f3142".to_string(), betu.to_string());
    }

    #[test]
    fn test_try_from_str() {
        let betu = BaseTagged::try_from("f3142").unwrap();
        assert_eq!(BaseTagged::default(), betu);
    }
}
