//! StreamBit Vision - High-performance image and video processing
//!
//! This module provides parallel image loading, resizing, and video frame extraction.
//! All operations are optimized for maximum throughput using Rayon for parallelism
//! and the `image` crate for pure-Rust image decoding.
//!
//! # Features
//!
//! - **Parallel Image Loading**: Load and process multiple images simultaneously
//! - **SIMD-Accelerated Resizing**: Fast image resizing using optimized algorithms
//! - **Multiple Formats**: Support for JPEG, PNG, WebP, and more
//! - **Zero-Copy Output**: Direct conversion to ndarray tensors
//! - **Batch Processing**: Efficient batch operations for ML workflows
//!
//! # Examples
//!
//! ```rust,no_run
//! use streambit_vision::ImageProcessor;
//!
//! let processor = ImageProcessor::new();
//! let images = processor.load_batch(
//!     vec!["img1.jpg", "img2.jpg", "img3.jpg"],
//!     Some((224, 224)),
//!     None
//! ).unwrap();
//! ```

pub mod image_proc;
pub mod enhancements;
pub mod filters;
pub mod batch_ops;

// Re-export main types
pub use image_proc::{ImageProcessor, ResizeMode};
pub use enhancements::{
    adjust_brightness, adjust_contrast, rotate, flip_horizontal, flip_vertical,
    crop, to_grayscale, RotationAngle
};
pub use filters::{gaussian_blur, sharpen, edge_detection, emboss};
pub use batch_ops::{apply_watermark, normalize_colors, auto_enhance};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
