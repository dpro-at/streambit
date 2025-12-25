//! Image processing module with parallel loading and resizing
//!
//! This module provides the `ImageProcessor` struct which handles batch image
//! loading with parallel processing using Rayon.

use image::{DynamicImage, GenericImageView, ImageError, ImageFormat};
use ndarray::{Array3, Array4, Axis};
use rayon::prelude::*;
use std::path::Path;
use streambit_core::{Result, StreamBitError, Tensor, TensorBatch};

/// Resize mode for image processing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeMode {
    /// Nearest neighbor (fastest, lowest quality)
    Nearest,
    /// Bilinear interpolation (good balance)
    Bilinear,
    /// Bicubic interpolation (slower, higher quality)
    Bicubic,
    /// Lanczos3 (slowest, highest quality)
    Lanczos3,
}

impl Default for ResizeMode {
    fn default() -> Self {
        ResizeMode::Bilinear
    }
}

impl ResizeMode {
    fn to_filter_type(&self) -> image::imageops::FilterType {
        match self {
            ResizeMode::Nearest => image::imageops::FilterType::Nearest,
            ResizeMode::Bilinear => image::imageops::FilterType::Triangle,
            ResizeMode::Bicubic => image::imageops::FilterType::CatmullRom,
            ResizeMode::Lanczos3 => image::imageops::FilterType::Lanczos3,
        }
    }
}

/// Image processor for parallel batch loading and processing
///
/// This struct provides methods for loading and processing images in parallel.
/// It's optimized for ML workflows where you need to load many images quickly.
///
/// # Examples
///
/// ```rust,no_run
/// use streambit_vision::ImageProcessor;
///
/// let processor = ImageProcessor::new();
/// let images = processor.load_batch(
///     vec!["img1.jpg", "img2.jpg"],
///     Some((224, 224)),
///     None
/// ).unwrap();
/// ```
pub struct ImageProcessor {
    resize_mode: ResizeMode,
    num_threads: Option<usize>,
}

impl Default for ImageProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageProcessor {
    /// Create a new image processor with default settings
    pub fn new() -> Self {
        Self {
            resize_mode: ResizeMode::default(),
            num_threads: None,
        }
    }

    /// Set the resize mode
    pub fn with_resize_mode(mut self, mode: ResizeMode) -> Self {
        self.resize_mode = mode;
        self
    }

    /// Set the number of threads for parallel processing
    pub fn with_threads(mut self, num_threads: usize) -> Self {
        self.num_threads = Some(num_threads);
        self
    }

    /// Load a single image from a file path
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the image file
    /// * `target_size` - Optional target size (width, height) for resizing
    ///
    /// # Returns
    ///
    /// A `Tensor` containing the image data in CHW format (Channels × Height × Width)
    pub fn load_image<P: AsRef<Path>>(
        &self,
        path: P,
        target_size: Option<(u32, u32)>,
    ) -> Result<Tensor> {
        let img = image::open(path.as_ref()).map_err(|e| {
            StreamBitError::Image(format!("Failed to load image: {}", e))
        })?;

        let processed = self.process_image(img, target_size)?;
        Ok(processed)
    }

    /// Load multiple images in parallel
    ///
    /// This is the main method for batch image loading. It processes all images
    /// in parallel using Rayon, maximizing CPU utilization.
    ///
    /// # Arguments
    ///
    /// * `paths` - Vector of image file paths
    /// * `target_size` - Optional target size (width, height) for resizing
    /// * `num_threads` - Optional number of threads (overrides instance setting)
    ///
    /// # Returns
    ///
    /// A `TensorBatch` containing all loaded images
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use streambit_vision::ImageProcessor;
    ///
    /// let processor = ImageProcessor::new();
    /// let paths = vec!["img1.jpg", "img2.jpg", "img3.jpg"];
    /// let batch = processor.load_batch(paths, Some((224, 224)), None).unwrap();
    /// assert_eq!(batch.len(), 3);
    /// ```
    pub fn load_batch<P: AsRef<Path> + Send + Sync>(
        &self,
        paths: Vec<P>,
        target_size: Option<(u32, u32)>,
        num_threads: Option<usize>,
    ) -> Result<TensorBatch> {
        // Configure thread pool if specified
        if let Some(threads) = num_threads.or(self.num_threads) {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build_global()
                .ok(); // Ignore error if already initialized
        }

        // Process all images in parallel
        let tensors: Result<Vec<Tensor>> = paths
            .par_iter()
            .map(|path| self.load_image(path, target_size))
            .collect();

        let tensors = tensors?;
        Ok(tensors.into_iter().collect())
    }

    /// Process a loaded image (resize and convert to tensor)
    fn process_image(
        &self,
        img: DynamicImage,
        target_size: Option<(u32, u32)>,
    ) -> Result<Tensor> {
        // Resize if target size is specified
        let img = if let Some((width, height)) = target_size {
            img.resize_exact(width, height, self.resize_mode.to_filter_type())
        } else {
            img
        };

        // Convert to RGB (3 channels)
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();

        // Convert to ndarray in CHW format (Channels × Height × Width)
        let mut array = Array3::<f32>::zeros((3, height as usize, width as usize));

        for y in 0..height {
            for x in 0..width {
                let pixel = rgb_img.get_pixel(x, y);
                // Normalize to [0, 1] range
                array[[0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
                array[[1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
                array[[2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
            }
        }

        Ok(Tensor::from_chw(array))
    }

    /// Load images and stack them into a single 4D tensor (NCHW format)
    ///
    /// This is useful for batch processing in ML frameworks.
    ///
    /// # Arguments
    ///
    /// * `paths` - Vector of image file paths
    /// * `target_size` - Target size (width, height) - required for stacking
    /// * `num_threads` - Optional number of threads
    ///
    /// # Returns
    ///
    /// A 4D ndarray with shape (N, C, H, W) where:
    /// - N = number of images
    /// - C = number of channels (3 for RGB)
    /// - H = height
    /// - W = width
    pub fn load_batch_stacked<P: AsRef<Path> + Send + Sync>(
        &self,
        paths: Vec<P>,
        target_size: (u32, u32),
        num_threads: Option<usize>,
    ) -> Result<Array4<f32>> {
        let batch = self.load_batch(paths, Some(target_size), num_threads)?;
        let tensors = batch.into_vec();

        if tensors.is_empty() {
            return Err(StreamBitError::Image("No images loaded".to_string()));
        }

        // Stack all tensors into a 4D array
        let num_images = tensors.len();
        let first_shape = tensors[0].shape();
        
        if first_shape.len() != 3 {
            return Err(StreamBitError::Image(
                "Expected 3D tensors (CHW format)".to_string(),
            ));
        }

        let (c, h, w) = (first_shape[0], first_shape[1], first_shape[2]);
        let mut stacked = Array4::<f32>::zeros((num_images, c, h, w));

        for (i, tensor) in tensors.iter().enumerate() {
            let data = tensor.to_chw();
            stacked.index_axis_mut(Axis(0), i).assign(&data);
        }

        Ok(stacked)
    }

    /// Get image information without loading the full image
    ///
    /// This is useful for quickly checking image dimensions and format.
    pub fn get_image_info<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<(u32, u32, ImageFormat)> {
        let reader = image::io::Reader::open(path.as_ref()).map_err(|e| {
            StreamBitError::Image(format!("Failed to open image: {}", e))
        })?;

        let format = reader.format().ok_or_else(|| {
            StreamBitError::Image("Could not determine image format".to_string())
        })?;

        let dimensions = reader.into_dimensions().map_err(|e| {
            StreamBitError::Image(format!("Failed to read dimensions: {}", e))
        })?;

        Ok((dimensions.0, dimensions.1, format))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_mode() {
        assert_eq!(ResizeMode::default(), ResizeMode::Bilinear);
    }

    #[test]
    fn test_processor_creation() {
        let processor = ImageProcessor::new();
        assert_eq!(processor.resize_mode, ResizeMode::Bilinear);
        assert_eq!(processor.num_threads, None);
    }

    #[test]
    fn test_processor_builder() {
        let processor = ImageProcessor::new()
            .with_resize_mode(ResizeMode::Lanczos3)
            .with_threads(4);
        
        assert_eq!(processor.resize_mode, ResizeMode::Lanczos3);
        assert_eq!(processor.num_threads, Some(4));
    }
}
