use thiserror::Error;

/// Errors generated by the numeric type impls
#[derive(Clone, Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Multitrait decode error
    #[error(transparent)]
    Multtrait(#[from] multitrait::Error),

    /// Multicodec decode error
    #[error(transparent)]
    Multcodec(#[from] multicodec::Error),

    /// BaseEncoded error
    #[error(transparent)]
    BaseEncoded(#[from] BaseEncodedError),

    /// Custom error for inner types to use when nothing else works
    #[error("Custom error: {0}")]
    Custom(String),
}

impl Error {
    /// create a custom error instance
    pub fn custom(s: &str) -> Self {
        Error::Custom(s.to_string())
    }
}

/// Errors generated by the base encoding smart pointer
#[derive(Clone, Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum BaseEncodedError {
    /// Multibase decode error
    #[error(transparent)]
    Multibase(#[from] multibase::Error),

    /// Value decoding failed
    #[error("Failed to decode the tagged value")]
    ValueFailed,
}
