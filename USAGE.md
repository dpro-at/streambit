# StreamBit - Complete Usage Guide

## 📋 Table of Contents
- [Running the Web UI](#running-the-web-ui)
- [Using the CLI](#using-the-cli)
- [Running Benchmarks](#running-benchmarks)
- [Downloading Hugging Face Datasets](#downloading-hugging-face-datasets)
- [Practical Examples](#practical-examples)

---

## 🌐 Running the Web UI

### Quick Start
```bash
# Start the server (Release mode for maximum speed)
cargo run --release --package streambit-web-ui
```

Server will be available at: `http://localhost:8080`

### Available Features:
1. **Upload Files** - Upload images manually
2. **Select Folders** - Process entire folders
3. **Hugging Face** - Download and test standard datasets

---

## 🖥️ Using the CLI

### Basic Command
```bash
cargo run --release -p streambit-cli -- process-folder <FOLDER_PATH>
```

### Available Options

#### 1. Specify Dimensions
```bash
cargo run --release -p streambit-cli -- process-folder "C:\Users\YourName\Pictures" --width 512 --height 512
```

#### 2. Choose Resize Mode
```bash
# Options: nearest, bilinear, bicubic, lanczos3
cargo run --release -p streambit-cli -- process-folder "benchmarks\data\images" --mode lanczos3
```

#### 3. Save Processed Images (Visual Verification)
```bash
cargo run --release -p streambit-cli -- process-folder "test_images" --save-output "output/processed" --format png
```

#### 4. Apply Image Filters

**Brightness & Contrast:**
```bash
# Make images brighter and more contrasted
cargo run --release -p streambit-cli -- process-folder "images" --brightness 1.5 --contrast 1.3 --save-output "output/enhanced"
```

**Blur & Sharpen:**
```bash
# Apply gaussian blur
cargo run --release -p streambit-cli -- process-folder "images" --blur 2.5 --save-output "output/blurred"

# Sharpen images
cargo run --release -p streambit-cli -- process-folder "images" --sharpen --save-output "output/sharp"
```

**Transformations:**
```bash
# Rotate 90 degrees and flip horizontally
cargo run --release -p streambit-cli -- process-folder "images" --rotate 90 --flip-h --save-output "output/rotated"

# Convert to grayscale
cargo run --release -p streambit-cli -- process-folder "images" --grayscale --save-output "output/grayscale"
```

**Edge Detection:**
```bash
# Detect edges using Sobel operator
cargo run --release -p streambit-cli -- process-folder "images" --edge-detect --save-output "output/edges"
```

**Combine Multiple Filters:**
```bash
# Apply multiple filters at once
cargo run --release -p streambit-cli -- process-folder "images" \
  --brightness 1.2 \
  --contrast 1.1 \
  --sharpen \
  --width 512 \
  --height 512 \
  --save-output "output/final" \
  --format webp
```

**Clean Output Directory:**
```bash
# Remove old files before saving new ones
cargo run --release -p streambit-cli -- process-folder "images" \
  --save-output "output/processed" \
  --clean

# Clean is useful when you want fresh output without old files
cargo run --release -p streambit-cli -- process-folder "images" \
  --brightness 1.5 \
  --save-output "output/enhanced" \
  --clean \
  --format png
```

---

## 🎵 Audio Processing

### Basic Command
```bash
cargo run --release -p streambit-cli -- process-audio <FOLDER_PATH>
```

### Available Options

#### 1. Resample Audio
```bash
# Resample to 16kHz (common for speech recognition)
cargo run --release -p streambit-cli -- process-audio "audio_folder" --sample-rate 16000 --save-output "output/audio"

# Resample to 44.1kHz (CD quality)
cargo run --release -p streambit-cli -- process-audio "audio_folder" --sample-rate 44100 --save-output "output/audio"
```

#### 2. Convert to Mono
```bash
# Convert stereo to mono
cargo run --release -p streambit-cli -- process-audio "audio_folder" --mono --save-output "output/mono"
```

#### 3. Normalize Audio
```bash
# Normalize audio levels
cargo run --release -p streambit-cli -- process-audio "audio_folder" --normalize --save-output "output/normalized"
```

#### 4. Trim Silence
```bash
# Remove silence from start and end (threshold: 0.01)
cargo run --release -p streambit-cli -- process-audio "audio_folder" --trim-silence 0.01 --save-output "output/trimmed"
```

#### 5. Combine Multiple Operations
```bash
# Resample, convert to mono, normalize, and trim
cargo run --release -p streambit-cli -- process-audio "audio_folder" \
  --sample-rate 16000 \
  --mono \
  --normalize \
  --trim-silence 0.01 \
  --save-output "output/processed" \
  --clean
```

#### 6. Full Example
```bash
cargo run --release -p streambit-cli -- process-audio "output/datasets/audio" \
  --sample-rate 16000 \
  --mono \
  --normalize \
  --save-output "output/processed_audio" \
  --format wav
```

---

#### 5. Full Example with All Options (Images)
```bash
cargo run --release -p streambit-cli -- process-folder "benchmarks\data\images" --save-output "output_folder"
```

#### 4. Choose Output Format
```bash
# Options: jpg, png, webp, bmp (default: jpg)
cargo run --release -p streambit-cli -- process-folder "my_folder" --save-output "output" --format png
```

---

## 🎨 Advanced Image Features

StreamBit includes powerful image enhancement, filtering, and batch operations capabilities.

### Image Enhancements

#### Brightness & Contrast
```rust
use streambit_vision::{adjust_brightness, adjust_contrast};

// Increase brightness by 20%
let brighter = adjust_brightness(&img, 1.2);

// Increase contrast by 50%
let contrasted = adjust_contrast(&img, 1.5);
```

#### Rotation & Flipping
```rust
use streambit_vision::{rotate, flip_horizontal, flip_vertical, RotationAngle};

// Rotate 90 degrees
let rotated = rotate(&img, RotationAngle::Rotate90);

// Flip horizontally (mirror)
let flipped = flip_horizontal(&img);
```

#### Cropping & Grayscale
```rust
use streambit_vision::{crop, to_grayscale};

// Crop to region
let cropped = crop(&img, 100, 100, 500, 500)?;

// Convert to grayscale
let gray = to_grayscale(&img);
```

### Image Filters

#### Blur & Sharpen
```rust
use streambit_vision::{gaussian_blur, sharpen};

// Apply Gaussian blur
let blurred = gaussian_blur(&img, 2.5);

// Sharpen image
let sharp = sharpen(&img);
```

#### Edge Detection
```rust
use streambit_vision::edge_detection;

// Detect edges using Sobel operator
let edges = edge_detection(&img);
```

### Batch Operations

#### Watermarking
```rust
use streambit_vision::apply_watermark;

// Add watermark at position (50, 50) with 50% opacity
let watermarked = apply_watermark(&img, &watermark, 50, 50, 0.5)?;
```

#### Color Normalization & Auto-Enhance
```rust
use streambit_vision::{normalize_colors, auto_enhance};

// Normalize color values to full 0-255 range
let normalized = normalize_colors(&img);

// Automatic enhancement (brightness + contrast + normalization)
let enhanced = auto_enhance(&img);
```

### Complete Examples

#### Example 1: Simple Processing
```bash
cargo run --release -p streambit-cli -- process-folder "C:\Users\Dpro GmbH\Pictures\Screenshots"
```

#### Example 2: Process with Output Saving
```bash
cargo run --release -p streambit-cli -- process-folder "benchmarks\data\images" --width 224 --height 224 --save-output "processed_images"
```

#### Example 3: High Quality with Lanczos3
```bash
cargo run --release -p streambit-cli -- process-folder "my_photos" --width 1024 --height 1024 --mode lanczos3 --save-output "hq_output"
```

---

## 📊 Running Benchmarks

### Rust vs Python Comparison

```bash
# Full automatic comparison
.\benchmark_compare.ps1

# Compare on specific folder
.\benchmark_compare.ps1 -TargetFolder "C:\Users\YourName\Pictures"

# Compare on Hugging Face dataset
.\benchmark_compare.ps1 -TargetFolder "benchmarks\data\images"
```

### Expected Results
- **Python (Pillow)**: ~300-400 images/sec
- **StreamBit (Rust)**: ~3000-4000 images/sec
- **Speedup**: 8x-26x faster! 🚀

---

## 🍃 Downloading Hugging Face Datasets

### Via Web UI
1. Open `http://localhost:8080`
2. Go to **Hugging Face** tab
3. Select image count (100, 500, 1000)
4. Click **Download & Test**

### Via Python Directly
```bash
# Download 100 images (default)
python benchmarks\download_dataset.py

# Download 500 images
python benchmarks\download_dataset.py --limit 500

# Download 1000 images from different dataset
python benchmarks\download_dataset.py --limit 1000 --dataset "beans"
```

Images will be saved to: `benchmarks/data/images`

---

## 🎯 Practical Examples

### Scenario 1: Quick Test
```bash
# 1. Download 100 images
python benchmarks\download_dataset.py

# 2. Run comparison
.\benchmark_compare.ps1 -TargetFolder "benchmarks\data\images"
```

### Scenario 2: Process Your Personal Photos
```bash
# Process and save results
cargo run --release -p streambit-cli -- process-folder "C:\Users\YourName\Photos" --width 512 --height 512 --save-output "processed_photos"
```

### Scenario 3: Prepare ML Training Data
```bash
# Uniform resizing for PyTorch/TensorFlow models
cargo run --release -p streambit-cli -- process-folder "training_data" --width 224 --height 224 --mode bilinear
```

### Scenario 4: Comprehensive Test
```bash
# 1. Download 1000 images
python benchmarks\download_dataset.py --limit 1000

# 2. Process and save
cargo run --release -p streambit-cli -- process-folder "benchmarks\data\images" --save-output "verified_output"

# 3. Run benchmark
.\benchmark_compare.ps1 -TargetFolder "benchmarks\data\images"
```

### Scenario 5: Format Conversion
```bash
# Convert PNG to JPG
cargo run --release -p streambit-cli -- process-folder "png_images" --save-output "jpg_output" --format jpg

# Convert to WebP (smaller file size)
cargo run --release -p streambit-cli -- process-folder "photos" --save-output "webp_output" --format webp

# Convert to PNG (lossless)
cargo run --release -p streambit-cli -- process-folder "images" --save-output "png_output" --format png
```

### Scenario 6: Batch Processing with Resize
```bash
# Resize 1000 images to thumbnails (300x300) and save as WebP
cargo run --release -p streambit-cli -- process-folder "originals" --width 300 --height 300 --format webp --save-output "thumbnails"
```

---

## 🔧 Tips & Tricks

### 1. Maximum Speed
- Always use `--release` for maximum performance
- Debug mode is 5-10x slower!

### 2. Quality Verification
- Use `--save-output` to visually inspect results
- Compare original vs processed images

### 3. Choosing Resize Mode
- **Nearest**: Fastest, lowest quality
- **Bilinear**: Good balance (default)
- **Bicubic**: Higher quality, slightly slower
- **Lanczos3**: Highest quality, slowest

### 4. Memory Usage
- Each 224x224 RGB image = ~600 KB in memory
- 1000 images = ~600 MB
- System uses Zero-Copy to minimize consumption

### 5. Output Format Selection
- **JPG**: Smallest file size, lossy compression (best for photos)
- **PNG**: Lossless compression, larger files (best for graphics/screenshots)
- **WebP**: Modern format, smaller than JPG with better quality
- **BMP**: Uncompressed, largest files (best for editing)

---

## ❓ Troubleshooting

### Issue: "Python not found"
**Solution**: Use full Python path:
```bash
& "C:\Users\Dpro GmbH\AppData\Local\Programs\Python\Python312\python.exe" benchmarks\download_dataset.py
```

### Issue: "Port 8080 already in use"
**Solution**: Close previous server with `Ctrl+C` or change port in code

### Issue: Black images in output
**Solution**: Verify original images are valid and in supported format (JPG, PNG, WebP)

---

## 📚 Additional Resources

- **README.md**: Project overview
- **TASKS.md**: Task list and progress
- **doc/**: Detailed documentation

---

## 🚀 Current Features

StreamBit Vision Module includes:
1. ✅ **High-Speed Image Loading** - 3000+ images/sec
2. ✅ **Multi-Format Support** - JPG, PNG, WebP, BMP, GIF
3. ✅ **Format Conversion** - Convert between any supported formats
4. ✅ **Image Enhancements** - Brightness, contrast, rotation, flip, crop, grayscale
5. ✅ **Image Filters** - Blur, sharpen, edge detection, emboss
6. ✅ **Batch Operations** - Watermark, normalize, auto-enhance
7. ✅ **ML-Ready Output** - Direct tensor conversion for PyTorch/TensorFlow
8. ✅ **Web UI & CLI** - Multiple interfaces for different workflows

## 🔮 Coming Soon

- **Audio Module** - High-speed audio processing
- **Video Module** - Frame extraction and processing (requires FFmpeg)
- **Python Bindings** - Use StreamBit from Python directly
- **Text Module** - PDF, DOCX, CSV processing

---

**Created by**: StreamBit Team  
**Date**: 2025-12-25  
**Version**: v0.1.0-alpha  
**Status**: Vision Module Complete ✅

