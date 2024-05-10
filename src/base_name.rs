// SPDX-License-Idnetifier: Apache-2.0
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
        Base32PadLower => "Base32PadLower",
        Base32PadUpper => "Base32PadUpper",
        Base32HexLower => "Base32HexLower",
        Base32HexUpper => "Base32HexUpper",
        Base32HexPadLower => "Base32HexPadLower",
        Base32HexPadUpper => "Base32HexPadUpper",
        Base32Z => "Base32Z",
        Base36Lower => "Base36Lower",
        Base36Upper => "Base36Upper",
        Base58Flickr => "Base58Flickr",
        Base58Btc => "Base58Btc",
        Base64 => "Base64",
        Base64Pad => "Base64Pad",
        Base64Url => "Base64Url",
        Base64UrlPad => "Base64UrlPad",
    }
    .to_string()
}
