use std::fs::File;
use std::path::Path;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use streambit_core::StreamBitError;

/// Audio data structure
#[derive(Debug, Clone)]
pub struct AudioData {
    /// Audio samples (interleaved for multi-channel)
    pub samples: Vec<f32>,
    /// Sample rate in Hz
    pub sample_rate: u32,
    /// Number of channels
    pub channels: u16,
    /// Duration in seconds
    pub duration: f64,
}

/// Audio file loader
pub struct AudioLoader;

impl AudioLoader {
    /// Load an audio file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<AudioData, StreamBitError> {
        let path = path.as_ref();
        
        // Open the file
        let file = File::open(path)
            .map_err(|e| StreamBitError::Io(format!("Failed to open audio file: {}", e)))?;
        
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        
        // Create a hint to help the format registry guess the format
        let mut hint = Hint::new();
        if let Some(ext) = path.extension() {
            hint.with_extension(&ext.to_string_lossy());
        }
        
        // Probe the media source
        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
            .map_err(|e| StreamBitError::Audio(format!("Failed to probe audio: {}", e)))?;
        
        let mut format = probed.format;
        
        // Get the default track
        let track = format
            .default_track()
            .ok_or_else(|| StreamBitError::Audio("No audio track found".to_string()))?;
        
        let track_id = track.id;
        let sample_rate = track.codec_params.sample_rate
            .ok_or_else(|| StreamBitError::Audio("Sample rate not found".to_string()))?;
        let channels = track.codec_params.channels
            .ok_or_else(|| StreamBitError::Audio("Channel count not found".to_string()))?
            .count() as u16;
        
        // Create a decoder
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &DecoderOptions::default())
            .map_err(|e| StreamBitError::Audio(format!("Failed to create decoder: {}", e)))?;
        
        let mut samples = Vec::new();
        
        // Decode all packets
        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(_) => break,
            };
            
            // Skip packets that don't belong to the selected track
            if packet.track_id() != track_id {
                continue;
            }
            
            match decoder.decode(&packet) {
                Ok(decoded) => {
                    // Convert to f32 samples
                    let spec = *decoded.spec();
                    let duration = decoded.capacity() as u64;
                    
                    let mut sample_buf = SampleBuffer::<f32>::new(duration, spec);
                    sample_buf.copy_interleaved_ref(decoded);
                    
                    samples.extend_from_slice(sample_buf.samples());
                }
                Err(e) => {
                    eprintln!("Decode error: {}", e);
                    continue;
                }
            }
        }
        
        let duration = samples.len() as f64 / (sample_rate as f64 * channels as f64);
        
        Ok(AudioData {
            samples,
            sample_rate,
            channels,
            duration,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audio_data_structure() {
        let audio = AudioData {
            samples: vec![0.0; 44100],
            sample_rate: 44100,
            channels: 2,
            duration: 0.5,
        };
        
        assert_eq!(audio.sample_rate, 44100);
        assert_eq!(audio.channels, 2);
    }
}
