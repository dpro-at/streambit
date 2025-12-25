//! Image Processing Demo
//!
//! This example demonstrates the parallel image processing capabilities
//! of StreamBit Vision module.
//!
//! Run with: cargo run --example image_processing_demo

use streambit_vision::{ImageProcessor, ResizeMode};
use std::time::Instant;

fn main() {
    env_logger::init();

    println!("🚀 StreamBit Vision - Image Processing Demo\n");
    println!("=".repeat(60));

    // Create image processor
    let processor = ImageProcessor::new()
        .with_resize_mode(ResizeMode::Bilinear);

    println!("✅ Image Processor created");
    println!("   - Resize Mode: Bilinear");
    println!("   - Threads: Auto (all cores)\n");

    // Demo 1: Create test images programmatically
    println!("📝 Demo 1: Creating test images...");
    create_test_images();
    println!("   ✅ Created 3 test images (red, green, blue)\n");

    // Demo 2: Load single image
    println!("📝 Demo 2: Loading single image...");
    match processor.load_image("test_images/red.png", Some((224, 224))) {
        Ok(tensor) => {
            println!("   ✅ Loaded successfully!");
            println!("   - Shape: {:?}", tensor.shape());
            println!("   - Elements: {}", tensor.len());
        }
        Err(e) => {
            println!("   ❌ Error: {}", e);
            println!("   💡 Make sure test images exist. Run create_test_images() first.");
        }
    }
    println!();

    // Demo 3: Batch loading with parallel processing
    println!("📝 Demo 3: Batch loading (parallel)...");
    let image_paths = vec![
        "test_images/red.png",
        "test_images/green.png",
        "test_images/blue.png",
    ];

    let start = Instant::now();
    match processor.load_batch(image_paths.clone(), Some((224, 224)), None) {
        Ok(batch) => {
            let duration = start.elapsed();
            println!("   ✅ Batch loaded successfully!");
            println!("   - Images: {}", batch.len());
            println!("   - Time: {:.2}ms", duration.as_secs_f64() * 1000.0);
            println!("   - Throughput: {:.0} images/sec", 
                     batch.len() as f64 / duration.as_secs_f64());
        }
        Err(e) => {
            println!("   ❌ Error: {}", e);
        }
    }
    println!();

    // Demo 4: Stacked batch (4D tensor)
    println!("📝 Demo 4: Stacked batch (NCHW format)...");
    match processor.load_batch_stacked(image_paths, (224, 224), None) {
        Ok(stacked) => {
            println!("   ✅ Stacked tensor created!");
            println!("   - Shape: {:?} (N, C, H, W)", stacked.shape());
            println!("   - Total elements: {}", stacked.len());
            println!("   - Memory: ~{:.2} MB", 
                     (stacked.len() * 4) as f64 / 1_000_000.0);
        }
        Err(e) => {
            println!("   ❌ Error: {}", e);
        }
    }
    println!();

    // Demo 5: Image info (fast metadata reading)
    println!("📝 Demo 5: Quick image info...");
    match processor.get_image_info("test_images/red.png") {
        Ok((width, height, format)) => {
            println!("   ✅ Image info:");
            println!("   - Dimensions: {}x{}", width, height);
            println!("   - Format: {:?}", format);
        }
        Err(e) => {
            println!("   ❌ Error: {}", e);
        }
    }
    println!();

    // Demo 6: Different resize modes comparison
    println!("📝 Demo 6: Resize modes comparison...");
    let modes = vec![
        ResizeMode::Nearest,
        ResizeMode::Bilinear,
        ResizeMode::Bicubic,
        ResizeMode::Lanczos3,
    ];

    for mode in modes {
        let proc = ImageProcessor::new().with_resize_mode(mode);
        let start = Instant::now();
        
        if let Ok(_) = proc.load_image("test_images/red.png", Some((224, 224))) {
            let duration = start.elapsed();
            println!("   - {:?}: {:.2}ms", mode, duration.as_secs_f64() * 1000.0);
        }
    }
    println!();

    println!("=".repeat(60));
    println!("✅ Demo completed successfully!");
    println!("\n💡 Next steps:");
    println!("   - Try with your own images");
    println!("   - Experiment with different resize modes");
    println!("   - Test with larger batches for better parallelism");
}

/// Create test images programmatically
fn create_test_images() {
    use image::{RgbImage, Rgb};
    use std::fs;

    // Create test_images directory
    fs::create_dir_all("test_images").ok();

    // Create red image (256x256)
    let mut red_img = RgbImage::new(256, 256);
    for pixel in red_img.pixels_mut() {
        *pixel = Rgb([255, 0, 0]);
    }
    red_img.save("test_images/red.png").ok();

    // Create green image (256x256)
    let mut green_img = RgbImage::new(256, 256);
    for pixel in green_img.pixels_mut() {
        *pixel = Rgb([0, 255, 0]);
    }
    green_img.save("test_images/green.png").ok();

    // Create blue image (256x256)
    let mut blue_img = RgbImage::new(256, 256);
    for pixel in blue_img.pixels_mut() {
        *pixel = Rgb([0, 0, 255]);
    }
    blue_img.save("test_images/blue.png").ok();
}
