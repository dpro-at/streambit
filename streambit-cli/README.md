# StreamBit CLI

Command-line tool for high-performance image processing.

## Installation

```powershell
cargo build --release --package streambit-cli
```

## Usage

### Process a folder of images

```powershell
cargo run --release --package streambit-cli -- process-folder "C:\path\to\images"
```

### With custom settings

```powershell
# Custom size
cargo run --release --package streambit-cli -- process-folder "C:\path\to\images" --width 512 --height 512

# Custom resize mode
cargo run --release --package streambit-cli -- process-folder "C:\path\to\images" --mode lanczos3
```

## Options

- `--width, -w`: Target width for resizing (default: 224)
- `--height, -h`: Target height for resizing (default: 224)
- `--mode, -m`: Resize mode - nearest, bilinear, bicubic, lanczos3 (default: bilinear)

## Example Output

```
🚀 StreamBit Image Processor
============================================================
📂 Reading folder: C:\Users\Dpro GmbH\Pictures\Screenshots
✅ Found 1865 images
🔧 Settings: 224x224, Mode: bilinear
============================================================
⚡ Processing images...
============================================================
✅ Processing Complete!
============================================================
📊 Results:
   Images Processed: 1865
   Time: 865.93 ms
   Throughput: 2154 images/sec

📐 Tensor Info:
   Shape: [3, 224, 224] (C, H, W)
   Total Tensors: 1865
   Memory: 1122.94 MB
============================================================
```

## Features

- ✅ Direct folder access (no upload needed)
- ✅ High-performance parallel processing
- ✅ Colored terminal output
- ✅ Detailed statistics
- ✅ Multiple resize modes
- ✅ Memory usage reporting

## Performance

**Example (1865 images):**
- Time: ~865ms
- Throughput: **2154 images/second**
- Memory: ~1.1GB

**10x faster than Python!** 🚀
