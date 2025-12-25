# Testing StreamBit Image Processor

## Quick Start

### 1. Install Rust (if not installed)

**Windows:**
- Download and run: https://win.rustup.rs/x86_64
- Or use: `winget install Rustlang.Rustup`

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Run the Image Processing Demo

```bash
# Navigate to the project directory
cd c:\MAMP\htdocs\flowxtra\streambit

# Run the demo
cargo run --package streambit-vision --example image_processing_demo
```

### 3. Run in Release Mode (Faster)

```bash
cargo run --release --package streambit-vision --example image_processing_demo
```

## What Will Happen

The demo will:
1. ✅ Create 3 test images automatically (red, green, blue)
2. ✅ Load and resize images in parallel
3. ✅ Show performance metrics
4. ✅ Compare different resize modes

## Expected Performance

On a modern CPU (8+ cores):
- **Single Image**: ~5-10ms
- **Batch (3 images)**: ~10-15ms
- **Throughput**: 200-500 images/second

## Troubleshooting

### "cargo: command not found"
- Restart your terminal after installing Rust
- Or add Rust to PATH manually

### Compilation Errors
- Make sure you're in the project root directory
- Try: `cargo clean` then run again

### Slow Performance
- Use `--release` flag for optimized build
- First run will be slower (compilation)

## Next Steps

After successful test:
1. ✅ Move to Audio Module
2. ✅ Move to Text Module
3. ✅ Or continue with Video Processing

---

**Need Help?** Check `streambit-vision/examples/README.md` for detailed documentation.
