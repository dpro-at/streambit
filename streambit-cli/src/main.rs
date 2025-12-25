use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::time::Instant;
use streambit_vision::{ImageProcessor, ResizeMode};

#[derive(Parser)]
#[command(name = "streambit")]
#[command(about = "StreamBit - High-Performance Image Processor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Process all images in a folder
    ProcessFolder {
        /// Path to the folder containing images
        path: String,

        /// Target width for resizing (default: 224)
        #[arg(short, long, default_value = "224")]
        width: u32,

        /// Target height for resizing (default: 224)
        #[arg(short, long, default_value = "224")]
        height: u32,

        /// Resize mode: nearest, bilinear, bicubic, lanczos3 (default: bilinear)
        #[arg(short, long, default_value = "bilinear")]
        mode: String,

        /// Save processed images to output folder
        #[arg(short, long)]
        save_output: Option<String>,

        /// Output format: jpg, png, webp, bmp (default: jpg)
        #[arg(short, long, default_value = "jpg")]
        format: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::ProcessFolder {
            path,
            width,
            height,
            mode,
            save_output,
            format,
        } => {
            process_folder(&path, width, height, &mode, save_output.as_deref(), &format);
        }
    }
}

fn process_folder(folder_path: &str, width: u32, height: u32, mode_str: &str, save_output: Option<&str>, format_str: &str) {
    println!("{}", "🚀 StreamBit Image Processor".bright_cyan().bold());
    println!("{}", "=".repeat(60).bright_black());

    // Parse resize mode
    let resize_mode = match mode_str.to_lowercase().as_str() {
        "nearest" => ResizeMode::Nearest,
        "bilinear" => ResizeMode::Bilinear,
        "bicubic" => ResizeMode::Bicubic,
        "lanczos3" => ResizeMode::Lanczos3,
        _ => {
            eprintln!("{} Invalid resize mode. Using bilinear.", "⚠️".yellow());
            ResizeMode::Bilinear
        }
    };

    // Parse output format
    use image::ImageFormat;
    let (output_format, file_ext) = match format_str.to_lowercase().as_str() {
        "png" => (ImageFormat::Png, "png"),
        "webp" => (ImageFormat::WebP, "webp"),
        "bmp" => (ImageFormat::Bmp, "bmp"),
        "jpg" | "jpeg" => (ImageFormat::Jpeg, "jpg"),
        _ => {
            eprintln!("{} Invalid format '{}'. Using jpg.", "⚠️".yellow(), format_str);
            (ImageFormat::Jpeg, "jpg")
        }
    };

    // Read folder
    println!("📂 Reading folder: {}", folder_path.bright_white());
    let mut image_files = Vec::new();

    match fs::read_dir(folder_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if matches!(ext_str.as_str(), "jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp") {
                        if let Some(path_str) = path.to_str() {
                            image_files.push(path_str.to_string());
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{} Failed to read folder: {}", "❌".red(), e);
            std::process::exit(1);
        }
    }

    if image_files.is_empty() {
        eprintln!("{} No images found in folder", "❌".red());
        std::process::exit(1);
    }

    println!("✅ Found {} images", image_files.len().to_string().bright_green());
    println!("🔧 Settings: {}x{}, Mode: {}", width, height, mode_str);
    
    if let Some(output_dir) = save_output {
        println!("💾 Output directory: {}", output_dir.bright_yellow());
        println!("📄 Output format: {}", file_ext.to_uppercase().bright_cyan());
        // Create output directory
        if let Err(e) = fs::create_dir_all(output_dir) {
            eprintln!("{} Failed to create output directory: {}", "❌".red(), e);
            std::process::exit(1);
        }
    }
    
    println!("{}", "=".repeat(60).bright_black());

    // Process images
    let start = Instant::now();
    let processor = ImageProcessor::new().with_resize_mode(resize_mode);

    match processor.load_batch(image_files.clone(), Some((width, height)), None) {
        Ok(batch) => {
            let duration = start.elapsed();
            let time_ms = duration.as_secs_f64() * 1000.0;
            let throughput = batch.len() as f64 / duration.as_secs_f64();

            println!("{}", "✅ Processing Complete!".bright_green().bold());
            println!("{}", "=".repeat(60).bright_black());
            println!("📊 {} Processed: {}", "Images".bright_white(), batch.len().to_string().bright_cyan());
            println!("⏱️  {} {:.2} ms", "Time:".bright_white(), time_ms.to_string().bright_cyan());
            println!("🚀 {} {:.2} images/sec", "Throughput:".bright_white(), throughput.to_string().bright_green().bold());
            
            println!("\n📐 Tensor Info:");
            if let Some(first) = batch.tensors().first() {
                let shape = first.shape();
                println!("   Shape: [{}, {}, {}] (C, H, W)", shape[0], shape[1], shape[2]);
            }
            println!("   Total Tensors: {}", batch.len());
            let total_elements: usize = batch.tensors().iter().map(|t| t.len()).sum();
            let memory_mb = (total_elements * 4) as f64 / 1_000_000.0;
            println!("   Memory: {:.2} MB", memory_mb);
            
            // Save images if output directory specified
            if let Some(output_dir) = save_output {
                println!("{}", "=".repeat(60).bright_black());
                println!("{}", "💾 Saving processed images...".bright_yellow());
                
                use image::{ImageBuffer, Rgb};
                
                for (idx, tensor) in batch.tensors().iter().enumerate() {
                    // Convert from CHW to HWC format for saving
                    let shape = tensor.shape();
                    let (_c, h, w) = (shape[0], shape[1], shape[2]);
                    
                    let mut img_buffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w as u32, h as u32);
                    
                    let data = tensor.data();
                    
                    for y in 0..h {
                        for x in 0..w {
                            let r = (data[[0, y, x]] * 255.0) as u8;
                            let g = (data[[1, y, x]] * 255.0) as u8;
                            let b = (data[[2, y, x]] * 255.0) as u8;
                            img_buffer.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
                        }
                    }
                    
                    let output_path = format!("{}/processed_{:04}.{}", output_dir, idx, file_ext);
                    
                    // Save with specific format
                    let save_result = img_buffer.save_with_format(&output_path, output_format);
                    
                    if let Err(e) = save_result {
                        eprintln!("{} Failed to save {}: {}", "⚠️".yellow(), output_path, e);
                    } else {
                        if idx < 5 {
                            println!("  ✓ Saved: {}", output_path.bright_white());
                        }
                    }
                }
                
                println!("✅ Saved {} images to {} as {}", batch.len(), output_dir.bright_green(), file_ext.to_uppercase().bright_cyan());
            }
            
            println!("{}", "=".repeat(60).bright_black());
        }
        Err(e) => {
            eprintln!("{} Processing failed: {}", "❌".red(), e);
            std::process::exit(1);
        }
    }
}
