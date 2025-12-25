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
cargo run --release -p streambit-cli -- process-folder "benchmarks\data\images" --save-output "output_folder"
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

## 🚀 Next Steps

After mastering image processing, you can:
1. Explore **Audio Module** (coming soon)
2. Integrate StreamBit with PyTorch/TensorFlow
3. Build production applications with blazing speed

---

**Created by**: StreamBit Team  
**Date**: 2025-12-25  
**Version**: v0.1.0-alpha
