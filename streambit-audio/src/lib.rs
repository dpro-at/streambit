//! StreamBit Audio Processing Module
//! 
//! High-performance audio processing library for:
//! - Loading audio files (MP3, WAV, FLAC, OGG)
//! - Resampling and format conversion
//! - Feature extraction (Mel Spectrogram, MFCC)
//! - Batch processing

pub mod loader;
pub mod processor;
pub mod features;
pub mod batch;

pub use loader::AudioLoader;
pub use processor::AudioProcessor;
pub use features::FeatureExtractor;
pub use batch::AudioBatch;

// Re-export common types
pub use streambit_core::StreamBitError;
