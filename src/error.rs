use core::fmt::{self, Display};

/// Errors that can occur during cryptographic operations.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    /// The output buffer was too small for the requested operation.
    InvalidBufferLength,
    /// Parameter length (e.g. key/nonce) exceeded the maximum absorb rate.
    InvalidParameterLength,
    /// An operation requiring a key (like encryption) was called on an unkeyed instance.
    KeyRequired,
    /// Decryption tag verification failed (ciphertext is corrupted or key/nonce is invalid).
    TagMismatch,
}
impl core::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidBufferLength => write!(f, "Invalid buffer length"),
            Error::InvalidParameterLength => write!(f, "Key too long"),
            Error::KeyRequired => write!(f, "A key is required"),
            Error::TagMismatch => write!(f, "Tag mismatch"),
        }
    }
}
