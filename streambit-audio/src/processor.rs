use crate::loader::AudioData;
use streambit_core::StreamBitError;
use rubato::{Resampler, SincFixedIn, InterpolationType, InterpolationParameters, WindowFunction};

/// Audio processor for transformations
pub struct AudioProcessor;

impl AudioProcessor {
    /// Resample audio to a target sample rate
    pub fn resample(audio: &AudioData, target_rate: u32) -> Result<AudioData, StreamBitError> {
        if audio.sample_rate == target_rate {
            return Ok(audio.clone());
        }
        
        let channels = audio.channels as usize;
        let ratio = target_rate as f64 / audio.sample_rate as f64;
        
        // Deinterleave samples
        let mut channel_data: Vec<Vec<f32>> = vec![Vec::new(); channels];
        for (i, &sample) in audio.samples.iter().enumerate() {
            channel_data[i % channels].push(sample);
        }
        
        // Create resampler
        let params = InterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: InterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        
        let mut resampler = SincFixedIn::<f32>::new(
            ratio,
            2.0,
            params,
            channel_data[0].len(),
            channels,
        ).map_err(|e| StreamBitError::Audio(format!("Resampler error: {}", e)))?;
        
        // Resample each channel
        let resampled = resampler.process(&channel_data, None)
            .map_err(|e| StreamBitError::Audio(format!("Resample failed: {}", e)))?;
        
        // Interleave samples
        let mut samples = Vec::new();
        let frame_count = resampled[0].len();
        for i in 0..frame_count {
            for ch in 0..channels {
                samples.push(resampled[ch][i]);
            }
        }
        
        let duration = samples.len() as f64 / (target_rate as f64 * channels as f64);
        
        Ok(AudioData {
            samples,
            sample_rate: target_rate,
            channels: audio.channels,
            duration,
        })
    }
    
    /// Convert stereo to mono by averaging channels
    pub fn to_mono(audio: &AudioData) -> Result<AudioData, StreamBitError> {
        if audio.channels == 1 {
            return Ok(audio.clone());
        }
        
        let channels = audio.channels as usize;
        let frame_count = audio.samples.len() / channels;
        let mut mono_samples = Vec::with_capacity(frame_count);
        
        for i in 0..frame_count {
            let mut sum = 0.0;
            for ch in 0..channels {
                sum += audio.samples[i * channels + ch];
            }
            mono_samples.push(sum / channels as f32);
        }
        
        let duration = mono_samples.len() as f64 / audio.sample_rate as f64;
        
        Ok(AudioData {
            samples: mono_samples,
            sample_rate: audio.sample_rate,
            channels: 1,
            duration,
        })
    }
    
    /// Normalize audio to [-1.0, 1.0] range
    pub fn normalize(audio: &mut AudioData) {
        let max_val = audio.samples.iter()
            .map(|&s| s.abs())
            .fold(0.0f32, f32::max);
        
        if max_val > 0.0 {
            for sample in &mut audio.samples {
                *sample /= max_val;
            }
        }
    }
    
    /// Trim silence from start and end
    pub fn trim_silence(audio: &AudioData, threshold: f32) -> Result<AudioData, StreamBitError> {
        let mut start = 0;
        let mut end = audio.samples.len();
        
        // Find start
        for (i, &sample) in audio.samples.iter().enumerate() {
            if sample.abs() > threshold {
                start = i;
                break;
            }
        }
        
        // Find end
        for (i, &sample) in audio.samples.iter().enumerate().rev() {
            if sample.abs() > threshold {
                end = i + 1;
                break;
            }
        }
        
        let samples = audio.samples[start..end].to_vec();
        let duration = samples.len() as f64 / (audio.sample_rate as f64 * audio.channels as f64);
        
        Ok(AudioData {
            samples,
            sample_rate: audio.sample_rate,
            channels: audio.channels,
            duration,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_to_mono() {
        let stereo = AudioData {
            samples: vec![1.0, 2.0, 3.0, 4.0], // 2 frames, 2 channels
            sample_rate: 44100,
            channels: 2,
            duration: 0.0,
        };
        
        let mono = AudioProcessor::to_mono(&stereo).unwrap();
        assert_eq!(mono.channels, 1);
        assert_eq!(mono.samples.len(), 2);
        assert_eq!(mono.samples[0], 1.5); // (1.0 + 2.0) / 2
        assert_eq!(mono.samples[1], 3.5); // (3.0 + 4.0) / 2
    }
    
    #[test]
    fn test_normalize() {
        let mut audio = AudioData {
            samples: vec![0.5, -1.0, 0.25, -0.75],
            sample_rate: 44100,
            channels: 1,
            duration: 0.0,
        };
        
        AudioProcessor::normalize(&mut audio);
        
        let max = audio.samples.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);
        assert!((max - 1.0).abs() < 0.001);
    }
}
