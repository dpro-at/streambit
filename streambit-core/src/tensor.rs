//! Tensor types and utilities
//!
//! This module provides tensor types that wrap ndarray for use across all
//! StreamBit modules. Tensors are the primary data structure for passing
//! data to Python AI frameworks.

use ndarray::{Array3, ArrayD, Ix3};

/// Unified tensor type wrapping ndarray
///
/// This provides a common interface for all data modalities.
/// Tensors use f32 (32-bit float) as the default data type for compatibility
/// with most AI/ML frameworks.
///
/// # Examples
///
/// ```rust
/// use streambit_core::Tensor;
/// use ndarray::Array3;
///
/// // Create a tensor from a 3D array (CHW format)
/// let data = Array3::<f32>::zeros((3, 224, 224));
/// let tensor = Tensor::from_chw(data);
/// assert_eq!(tensor.shape(), &[3, 224, 224]);
/// ```
#[derive(Debug, Clone)]
pub struct Tensor {
    /// Internal ndarray storage (dynamic dimensions)
    data: ArrayD<f32>,
}

impl Tensor {
    /// Create a new tensor from an ndarray
    pub fn new(data: ArrayD<f32>) -> Self {
        Self { data }
    }

    /// Create a tensor from a 3D array (CHW format)
    ///
    /// CHW format means: Channels × Height × Width
    /// This is the standard format for PyTorch and many other frameworks.
    pub fn from_chw(data: Array3<f32>) -> Self {
        Self {
            data: data.into_dyn(),
        }
    }

    /// Get the shape of the tensor
    pub fn shape(&self) -> &[usize] {
        self.data.shape()
    }

    /// Get a reference to the underlying data
    pub fn data(&self) -> &ArrayD<f32> {
        &self.data
    }

    /// Get a mutable reference to the underlying data
    pub fn data_mut(&mut self) -> &mut ArrayD<f32> {
        &mut self.data
    }

    /// Convert to a 3D array (CHW format)
    ///
    /// # Panics
    /// Panics if the tensor is not 3-dimensional
    pub fn to_chw(&self) -> Array3<f32> {
        self.data
            .clone()
            .into_dimensionality::<Ix3>()
            .expect("Tensor must be 3-dimensional for CHW conversion")
    }

    /// Get the number of elements in the tensor
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the tensor is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the number of dimensions
    pub fn ndim(&self) -> usize {
        self.data.ndim()
    }
}

/// Batch of tensors for efficient processing
///
/// This type is used to group multiple tensors together for batch processing.
/// It's commonly used when loading multiple images, audio files, etc.
#[derive(Debug, Clone)]
pub struct TensorBatch {
    /// List of tensors in the batch
    tensors: Vec<Tensor>,
}

impl TensorBatch {
    /// Create a new empty batch
    pub fn new() -> Self {
        Self {
            tensors: Vec::new(),
        }
    }

    /// Create a batch with a specific capacity
    ///
    /// This pre-allocates space for the given number of tensors,
    /// which can improve performance when the batch size is known.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tensors: Vec::with_capacity(capacity),
        }
    }

    /// Add a tensor to the batch
    pub fn push(&mut self, tensor: Tensor) {
        self.tensors.push(tensor);
    }

    /// Get the number of tensors in the batch
    pub fn len(&self) -> usize {
        self.tensors.len()
    }

    /// Check if the batch is empty
    pub fn is_empty(&self) -> bool {
        self.tensors.is_empty()
    }

    /// Get a reference to the tensors
    pub fn tensors(&self) -> &[Tensor] {
        &self.tensors
    }

    /// Get a mutable reference to the tensors
    pub fn tensors_mut(&mut self) -> &mut [Tensor] {
        &mut self.tensors
    }

    /// Convert to a vector of tensors
    pub fn into_vec(self) -> Vec<Tensor> {
        self.tensors
    }

    /// Get a tensor by index
    pub fn get(&self, index: usize) -> Option<&Tensor> {
        self.tensors.get(index)
    }
}

impl Default for TensorBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<Tensor> for TensorBatch {
    fn from_iter<T: IntoIterator<Item = Tensor>>(iter: T) -> Self {
        Self {
            tensors: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array3;

    #[test]
    fn test_tensor_creation() {
        let data = Array3::<f32>::zeros((3, 224, 224));
        let tensor = Tensor::from_chw(data);
        assert_eq!(tensor.shape(), &[3, 224, 224]);
        assert_eq!(tensor.ndim(), 3);
        assert_eq!(tensor.len(), 3 * 224 * 224);
    }

    #[test]
    fn test_tensor_batch() {
        let mut batch = TensorBatch::with_capacity(2);
        let t1 = Tensor::from_chw(Array3::<f32>::zeros((3, 224, 224)));
        let t2 = Tensor::from_chw(Array3::<f32>::zeros((3, 224, 224)));
        
        batch.push(t1);
        batch.push(t2);
        
        assert_eq!(batch.len(), 2);
        assert!(!batch.is_empty());
    }
}
