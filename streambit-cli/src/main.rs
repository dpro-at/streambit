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
        } => {
            process_folder(&path, width, height, &mode);
        }
    }
}

fn process_folder(folder_path: &str, width: u32, height: u32, mode_str: &str) {
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
    println!("{}", "=".repeat(60).bright_black());

    // Process images
    println!("⚡ Processing images...");
    let start = Instant::now();

    let processor = ImageProcessor::new().with_resize_mode(resize_mode);

    match processor.load_batch(image_files.clone(), Some((width, height)), None) {
        Ok(batch) => {
            let duration = start.elapsed();
            let time_ms = duration.as_secs_f64() * 1000.0;
            let throughput = batch.len() as f64 / duration.as_secs_f64();

            println!("{}", "=".repeat(60).bright_black());
            println!("{} {}", "✅".green(), "Processing Complete!".bright_green().bold());
            println!("{}", "=".repeat(60).bright_black());
            
            println!("📊 Results:");
            println!("   Images Processed: {}", batch.len().to_string().bright_cyan());
            println!("   Time: {} ms", format!("{:.2}", time_ms).bright_yellow());
            println!("   Throughput: {} images/sec", format!("{:.0}", throughput).bright_magenta());
            
            println!("\n📐 Tensor Info:");
            println!("   Shape: {:?} (C, H, W)", batch.tensors()[0].shape());
            println!("   Total Tensors: {}", batch.len());
            
            let total_elements: usize = batch.tensors().iter().map(|t| t.len()).sum();
            let memory_mb = (total_elements * 4) as f64 / 1_000_000.0;
            println!("   Memory: {:.2} MB", memory_mb);
            
            println!("{}", "=".repeat(60).bright_black());
        }
        Err(e) => {
            eprintln!("{} Error processing images: {}", "❌".red(), e);
            std::process::exit(1);
        }
    }
}
