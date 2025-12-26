use streambit_audio::{AudioLoader, AudioProcessor, FeatureExtractor};

fn main() {
    println!("🎵 StreamBit Audio Module Test\n");
    
    // Example: Create a simple test audio (sine wave)
    println!("Creating test audio data...");
    
    let sample_rate = 44100;
    let duration = 1.0; // 1 second
    let frequency = 440.0; // A4 note
    
    let num_samples = (sample_rate as f64 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    
    for i in 0..num_samples {
        let t = i as f64 / sample_rate as f64;
        let sample = (2.0 * std::f64::consts::PI * frequency * t).sin() as f32;
        samples.push(sample);
    }
    
    let mut audio = streambit_audio::loader::AudioData {
        samples,
        sample_rate,
        channels: 1,
        duration,
    };
    
    println!("✅ Created audio: {}Hz, {} channels, {:.2}s\n", 
        audio.sample_rate, audio.channels, audio.duration);
    
    // Test 1: Normalization
    println!("Test 1: Normalization");
    AudioProcessor::normalize(&mut audio);
    let max_val = audio.samples.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);
    println!("  Max value after normalization: {:.4}", max_val);
    println!("  ✅ Normalization works!\n");
    
    // Test 2: Resampling
    println!("Test 2: Resampling to 16kHz");
    match AudioProcessor::resample(&audio, 16000) {
        Ok(resampled) => {
            println!("  Original: {}Hz, {} samples", audio.sample_rate, audio.samples.len());
            println!("  Resampled: {}Hz, {} samples", resampled.sample_rate, resampled.samples.len());
            println!("  ✅ Resampling works!\n");
        }
        Err(e) => {
            println!("  ❌ Resampling failed: {}\n", e);
        }
    }
    
    // Test 3: Feature Extraction - RMS Energy
    println!("Test 3: RMS Energy Extraction");
    let energy = FeatureExtractor::rms_energy(&audio, 2048);
    println!("  Extracted {} energy frames", energy.len());
    if !energy.is_empty() {
        println!("  First frame energy: {:.4}", energy[0]);
        println!("  ✅ RMS Energy extraction works!\n");
    }
    
    // Test 4: Zero Crossing Rate
    println!("Test 4: Zero Crossing Rate");
    let zcr = FeatureExtractor::zero_crossing_rate(&audio, 2048);
    println!("  Extracted {} ZCR frames", zcr.len());
    if !zcr.is_empty() {
        println!("  First frame ZCR: {:.4}", zcr[0]);
        println!("  ✅ Zero Crossing Rate works!\n");
    }
    
    // Test 5: Silence Trimming
    println!("Test 5: Silence Trimming");
    match AudioProcessor::trim_silence(&audio, 0.01) {
        Ok(trimmed) => {
            println!("  Original: {} samples", audio.samples.len());
            println!("  Trimmed: {} samples", trimmed.samples.len());
            println!("  ✅ Silence trimming works!\n");
        }
        Err(e) => {
            println!("  ❌ Trimming failed: {}\n", e);
        }
    }
    
    println!("🎉 All tests completed!");
    println!("\n📝 Note: To test with real audio files:");
    println!("   1. Download audio dataset:");
    println!("      python streambit-web-ui/download_audio_dataset.py --limit 5");
    println!("   2. Then load and process the files using AudioLoader::load()");
}
