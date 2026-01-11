//! Encoding/decoding interfaces.

/// Base64 encoding/decoding.
pub trait Base64 {
    /// Encode bytes to base64 string.
    fn encode(data: &[u8]) -> String;

    /// Decode base64 string to bytes.
    fn decode(encoded: &str) -> Result<Vec<u8>, DecodeError>;
}

/// URL-safe Base64 encoding/decoding (no padding).
pub trait Base64Url {
    /// Encode bytes to URL-safe base64 string.
    fn encode(data: &[u8]) -> String;

    /// Decode URL-safe base64 string to bytes.
    fn decode(encoded: &str) -> Result<Vec<u8>, DecodeError>;
}

/// Hexadecimal encoding/decoding.
pub trait Hex {
    /// Encode bytes to hex string.
    fn encode(data: &[u8]) -> String;

    /// Encode bytes to uppercase hex string.
    fn encode_upper(data: &[u8]) -> String;

    /// Decode hex string to bytes.
    fn decode(encoded: &str) -> Result<Vec<u8>, DecodeError>;
}

/// URL encoding/decoding (percent encoding).
pub trait UrlEncoding {
    /// Encode a string for use in URLs.
    fn encode(input: &str) -> String;

    /// Decode a URL-encoded string.
    fn decode(encoded: &str) -> Result<String, DecodeError>;
}

/// Decoding errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Invalid character in input.
    InvalidCharacter(char),
    /// Invalid length.
    InvalidLength,
    /// Invalid padding.
    InvalidPadding,
    /// Invalid UTF-8.
    InvalidUtf8,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCharacter(c) => write!(f, "invalid character: {:?}", c),
            Self::InvalidLength => write!(f, "invalid length"),
            Self::InvalidPadding => write!(f, "invalid padding"),
            Self::InvalidUtf8 => write!(f, "invalid UTF-8"),
        }
    }
}

impl std::error::Error for DecodeError {}
