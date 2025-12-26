use crate::loader::{AudioData, AudioLoader};
use streambit_core::StreamBitError;
use std::path::Path;

/// Batch of audio files
pub struct AudioBatch {
    audios: Vec<AudioData>,
}

impl AudioBatch {
    /// Create a new empty batch
    pub fn new() -> Self {
        Self {
            audios: Vec::new(),
        }
    }
    
    /// Load multiple audio files
    pub fn load_batch<P: AsRef<Path>>(paths: Vec<P>) -> Result<Self, StreamBitError> {
        let mut audios = Vec::new();
        
        for path in paths {
            let audio = AudioLoader::load(path)?;
            audios.push(audio);
        }
        
        Ok(Self { audios })
    }
    
    /// Get all audio data
    pub fn audios(&self) -> &[AudioData] {
        &self.audios
    }
    
    /// Get number of audio files in batch
    pub fn len(&self) -> usize {
        self.audios.len()
    }
    
    /// Check if batch is empty
    pub fn is_empty(&self) -> bool {
        self.audios.is_empty()
    }
    
    /// Add audio to batch
    pub fn push(&mut self, audio: AudioData) {
        self.audios.push(audio);
    }
}

impl Default for AudioBatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_batch_creation() {
        let batch = AudioBatch::new();
        assert_eq!(batch.len(), 0);
        assert!(batch.is_empty());
    }
    
    #[test]
    fn test_batch_push() {
        let mut batch = AudioBatch::new();
        
        let audio = AudioData {
            samples: vec![0.0; 1000],
            sample_rate: 44100,
            channels: 2,
            duration: 0.0,
        };
        
        batch.push(audio);
        assert_eq!(batch.len(), 1);
        assert!(!batch.is_empty());
    }
}
