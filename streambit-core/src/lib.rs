//! StreamBit Core - Shared utilities and types for all StreamBit modules
//!
//! This crate provides common functionality used across all StreamBit modules:
//! - Error handling with comprehensive error types
//! - Tensor types and conversions for AI/ML workflows
//! - Parallel processing utilities using Rayon
//! - Configuration types for performance tuning
//!
//! # Examples
//!
//! ```rust
//! use streambit_core::{Tensor, parallel};
//!
//! // Process data in parallel
//! let items = vec![1, 2, 3, 4, 5];
//! let results = parallel::process_parallel(items, |x| x * 2);
//! ```

pub mod error;
pub mod parallel;
pub mod tensor;

// Re-export commonly used types
pub use error::{Result, StreamBitError};
pub use tensor::{Tensor, TensorBatch};

/// StreamBit version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
