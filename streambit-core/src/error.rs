//! Error types for StreamBit
//!
//! This module provides a unified error type for all StreamBit operations.
//! All errors implement the standard Error trait and can be converted to/from
//! other error types using the `?` operator.

use thiserror::Error;

/// StreamBit error type
///
/// This enum covers all possible errors that can occur in StreamBit operations.
#[derive(Error, Debug)]
pub enum StreamBitError {
    /// IO error (file not found, permission denied, etc.)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Image processing error
    #[error("Image processing error: {0}")]
    Image(String),

    /// Audio processing error
    #[error("Audio processing error: {0}")]
    Audio(String),

    /// Text extraction error
    #[error("Text extraction error: {0}")]
    Text(String),

    /// Video processing error
    #[error("Video processing error: {0}")]
    Video(String),

    /// Tabular data error
    #[error("Tabular data error: {0}")]
    Tabular(String),

    /// Streaming error
    #[error("Streaming error: {0}")]
    Streaming(String),

    /// Invalid format error
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// Unsupported operation
    #[error("Unsupported operation: {0}")]
    Unsupported(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Generic error with custom message
    #[error("{0}")]
    Other(String),
}

/// Result type alias for StreamBit operations
///
/// This is a convenience type alias that uses StreamBitError as the error type.
pub type Result<T> = std::result::Result<T, StreamBitError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = StreamBitError::Image("test error".to_string());
        assert_eq!(err.to_string(), "Image processing error: test error");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let sb_err: StreamBitError = io_err.into();
        assert!(sb_err.to_string().contains("IO error"));
    }
}
