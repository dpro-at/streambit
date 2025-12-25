# StreamBit Vision - Image Processing Demo

This example demonstrates the parallel image processing capabilities of StreamBit.

## Prerequisites

Make sure you have Rust installed:
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or on Windows, download from: https://rustup.rs
```

## Running the Demo

### Option 1: Run from the vision module directory

```bash
cd streambit-vision
cargo run --example image_processing_demo
```

### Option 2: Run from the workspace root

```bash
cargo run --package streambit-vision --example image_processing_demo
```

## What the Demo Does

The demo showcases:

1. **Test Image Creation**: Automatically creates 3 test images (red, green, blue)
2. **Single Image Loading**: Loads and resizes a single image
3. **Batch Loading**: Loads multiple images in parallel
4. **Stacked Tensors**: Creates 4D tensors (NCHW format) for ML frameworks
5. **Image Info**: Quick metadata reading without loading full image
6. **Resize Modes**: Compares different resize algorithms

## Expected Output

```
🚀 StreamBit Vision - Image Processing Demo

============================================================
✅ Image Processor created
   - Resize Mode: Bilinear
   - Threads: Auto (all cores)

📝 Demo 1: Creating test images...
   ✅ Created 3 test images (red, green, blue)

📝 Demo 2: Loading single image...
   ✅ Loaded successfully!
   - Shape: [3, 224, 224]
   - Elements: 150528

📝 Demo 3: Batch loading (parallel)...
   ✅ Batch loaded successfully!
   - Images: 3
   - Time: X.XXms
   - Throughput: XXX images/sec

📝 Demo 4: Stacked batch (NCHW format)...
   ✅ Stacked tensor created!
   - Shape: [3, 3, 224, 224] (N, C, H, W)
   - Total elements: 451584
   - Memory: ~1.81 MB

📝 Demo 5: Quick image info...
   ✅ Image info:
   - Dimensions: 256x256
   - Format: Png

📝 Demo 6: Resize modes comparison...
   - Nearest: X.XXms
   - Bilinear: X.XXms
   - Bicubic: X.XXms
   - Lanczos3: X.XXms

============================================================
✅ Demo completed successfully!
```

## Testing with Your Own Images

1. Create a directory with your images:
```bash
mkdir my_images
# Add your images (JPEG, PNG, WebP supported)
```

2. Modify the demo code to use your images:
```rust
let image_paths = vec![
    "my_images/photo1.jpg",
    "my_images/photo2.jpg",
    "my_images/photo3.jpg",
];
```

## Performance Tips

- **Batch Size**: Larger batches benefit more from parallelism
- **Resize Mode**: 
  - `Nearest` - Fastest, lowest quality
  - `Bilinear` - Good balance (default)
  - `Bicubic` - Better quality, slower
  - `Lanczos3` - Best quality, slowest
- **Threads**: Auto-detects all CPU cores by default

## Troubleshooting

### Error: "Failed to load image"
- Check that the image file exists
- Verify the file format is supported (JPEG, PNG, WebP, etc.)
- Ensure you have read permissions

### Error: "No images loaded"
- Make sure the `test_images` directory exists
- Run the demo once to create test images automatically

### Slow Performance
- Try with larger batches (10+ images)
- Check CPU usage - should be near 100% on all cores
- Verify you're running in release mode: `cargo run --release --example image_processing_demo`

## Next Steps

After testing the image processor, you can:
1. Integrate with Python using PyO3 bindings
2. Add video processing support
3. Implement custom preprocessing pipelines
4. Benchmark against other libraries (OpenCV, Pillow)

## License

MIT License - See LICENSE file for details
