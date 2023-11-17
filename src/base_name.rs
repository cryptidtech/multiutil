use crate::prelude::Base;

/// convert a multibase Base to its string equivalent
pub fn base_name(b: Base) -> String {
    use Base::*;
    match b {
        Identity => "Identity",
        Base2 => "Base2",
        Base8 => "Base8",
        Base10 => "Base10",
        Base16Lower => "Base16Lower",
        Base16Upper => "Base16Upper",
        Base32Lower => "Base32Lower",
        Base32Upper => "Base32Upper",
        Base32PadLower => "Base32PaddedLower",
        Base32PadUpper => "Base32PaddedUpper",
        Base32HexLower => "Base32HexLower",
        Base32HexUpper => "Base32HexUpper",
        Base32HexPadLower => "Base32HexPaddedLower",
        Base32HexPadUpper => "Base32HexPaddedUpper",
        Base32Z => "Base32Z",
        Base36Lower => "Base36Lower",
        Base36Upper => "Base36Upper",
        Base58Flickr => "Base58Flickr",
        Base58Btc => "Base58Btc",
        Base64 => "Base64",
        Base64Pad => "Base64Padded",
        Base64Url => "Base64URLSafe",
        Base64UrlPad => "Base64URLSafePadded",
    }
    .to_string()
}
