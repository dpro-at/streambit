# StreamBit Audio 🎵

High-performance audio processing library for Rust.

## Features

- ✅ **Multi-format Support**: MP3, WAV, FLAC, OGG, and more
- ✅ **Resampling**: High-quality sample rate conversion
- ✅ **Channel Conversion**: Stereo ↔ Mono
- ✅ **Normalization**: Audio level normalization
- ✅ **Silence Trimming**: Remove silence from start/end
- ✅ **Feature Extraction**: RMS Energy, Zero Crossing Rate
- ✅ **Batch Processing**: Process multiple files efficiently
- 🚧 **Mel Spectrogram**: Coming soon
- 🚧 **MFCC**: Coming soon

## Usage

```rust
use streambit_audio::{AudioLoader, AudioProcessor};

// Load an audio file
let audio = AudioLoader::load("song.mp3")?;

// Resample to 16kHz
let resampled = AudioProcessor::resample(&audio, 16000)?;

// Convert to mono
let mono = AudioProcessor::to_mono(&resampled)?;

// Normalize
let mut normalized = mono;
AudioProcessor::normalize(&mut normalized);

// Trim silence
let trimmed = AudioProcessor::trim_silence(&normalized, 0.01)?;
```

## Batch Processing

```rust
use streambit_audio::AudioBatch;

let paths = vec!["audio1.mp3", "audio2.wav", "audio3.flac"];
let batch = AudioBatch::load_batch(paths)?;

println!("Loaded {} audio files", batch.len());
```

## Feature Extraction

```rust
use streambit_audio::FeatureExtractor;

// Calculate RMS energy
let energy = FeatureExtractor::rms_energy(&audio, 2048);

// Calculate zero crossing rate
let zcr = FeatureExtractor::zero_crossing_rate(&audio, 2048);
```

## Dependencies

- **symphonia**: Audio decoding
- **rubato**: High-quality resampling
- **hound**: WAV encoding

## License

MIT
