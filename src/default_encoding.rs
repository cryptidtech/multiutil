use crate::prelude::Base;

/// This trait exposes the preferred encoding for this multicodec type
pub trait DefaultEncoding {
    /// return the preferred encoding for this multicodec type
    fn encoding() -> Base;
}
