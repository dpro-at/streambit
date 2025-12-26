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
        #[arg(short = 'H', long, default_value = "224")]
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

        // === Enhancements ===
        /// Brightness adjustment (1.0 = no change, >1.0 = brighter, <1.0 = darker)
        #[arg(long)]
        brightness: Option<f32>,

        /// Contrast adjustment (1.0 = no change, >1.0 = more contrast, <1.0 = less contrast)
        #[arg(long)]
        contrast: Option<f32>,

        /// Rotation angle: 90, 180, 270
        #[arg(long)]
        rotate: Option<u16>,

        /// Flip horizontally
        #[arg(long)]
        flip_h: bool,

        /// Flip vertically
        #[arg(long)]
        flip_v: bool,

        /// Convert to grayscale
        #[arg(long)]
        grayscale: bool,

        // === Filters ===
        /// Gaussian blur strength (0.0 = no blur, typical: 0.5-10.0)
        #[arg(long)]
        blur: Option<f32>,

        /// Apply sharpen filter
        #[arg(long)]
        sharpen: bool,

        /// Apply edge detection
        #[arg(long)]
        edge_detect: bool,

        /// Clean output directory before saving (remove old files)
        #[arg(long)]
        clean: bool,
    },

    /// Process audio files in a folder
    ProcessAudio {
        /// Path to the folder containing audio files
        path: String,

        /// Target sample rate in Hz (e.g., 16000, 44100, 48000)
        #[arg(short, long)]
        sample_rate: Option<u32>,

        /// Convert to mono
        #[arg(long)]
        mono: bool,

        /// Normalize audio to [-1.0, 1.0] range
        #[arg(long)]
        normalize: bool,

        /// Trim silence from start and end (threshold: 0.0-1.0)
        #[arg(long)]
        trim_silence: Option<f32>,

        /// Save processed audio to output folder
        #[arg(short, long)]
        save_output: Option<String>,

        /// Output format: wav (default: wav)
        #[arg(short, long, default_value = "wav")]
        format: String,

        /// Clean output directory before saving
        #[arg(long)]
        clean: bool,
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
            brightness,
            contrast,
            rotate,
            flip_h,
            flip_v,
            grayscale,
            blur,
            sharpen,
            edge_detect,
            clean,
        } => {
            process_folder(
                &path,
                width,
                height,
                &mode,
                save_output.as_deref(),
                &format,
                brightness,
                contrast,
                rotate,
                flip_h,
                flip_v,
                grayscale,
                blur,
                sharpen,
                edge_detect,
                clean,
            );
        }
        Commands::ProcessAudio {
            path,
            sample_rate,
            mono,
            normalize,
            trim_silence,
            save_output,
            format,
            clean,
        } => {
            process_audio(
                &path,
                sample_rate,
                mono,
                normalize,
                trim_silence,
                save_output.as_deref(),
                &format,
                clean,
            );
        }
    }
}

fn process_folder(
    folder_path: &str,
    width: u32,
    height: u32,
    mode_str: &str,
    save_output: Option<&str>,
    format_str: &str,
    brightness: Option<f32>,
    contrast: Option<f32>,
    rotate: Option<u16>,
    flip_h: bool,
    flip_v: bool,
    grayscale: bool,
    blur: Option<f32>,
    sharpen: bool,
    edge_detect: bool,
    clean: bool,
) {
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
    
    // Show active filters
    let mut active_filters = Vec::new();
    if brightness.is_some() { active_filters.push(format!("Brightness: {}", brightness.unwrap())); }
    if contrast.is_some() { active_filters.push(format!("Contrast: {}", contrast.unwrap())); }
    if blur.is_some() { active_filters.push(format!("Blur: {}", blur.unwrap())); }
    if rotate.is_some() { active_filters.push(format!("Rotate: {}°", rotate.unwrap())); }
    if flip_h { active_filters.push("Flip H".to_string()); }
    if flip_v { active_filters.push("Flip V".to_string()); }
    if grayscale { active_filters.push("Grayscale".to_string()); }
    if sharpen { active_filters.push("Sharpen".to_string()); }
    if edge_detect { active_filters.push("Edge Detect".to_string()); }
    
    if !active_filters.is_empty() {
        println!("🎨 Filters: {}", active_filters.join(", ").bright_magenta());
    }
    
    if let Some(output_dir) = save_output {
        println!("💾 Output directory: {}", output_dir.bright_yellow());
        println!("📄 Output format: {}", file_ext.to_uppercase().bright_cyan());
        
        // Clean output directory if requested
        if clean {
            if std::path::Path::new(output_dir).exists() {
                println!("🧹 Cleaning old files from: {}", output_dir.bright_yellow());
                if let Err(e) = fs::remove_dir_all(output_dir) {
                    eprintln!("{} Failed to clean output directory: {}", "⚠️".yellow(), e);
                }
            }
        }
        
        // Create output directory
        if let Err(e) = fs::create_dir_all(output_dir) {
            eprintln!("{} Failed to create output directory: {}", "❌".red(), e);
            std::process::exit(1);
        }
    }
    
    println!("{}", "=".repeat(60).bright_black());

    // Apply filters to images if any are specified
    let mut processed_files = image_files.clone();
    
    if !active_filters.is_empty() {
        println!("{}", "🎨 Applying filters...".bright_magenta());
        use streambit_vision::{enhancements, filters};
        use image;
        
        // Create temp directory for filtered images
        let temp_dir = "output/temp/cli_filters";
        
        // Clean temp directory if requested
        if clean && std::path::Path::new(temp_dir).exists() {
            if let Err(e) = fs::remove_dir_all(temp_dir) {
                eprintln!("{} Failed to clean temp directory: {}", "⚠️".yellow(), e);
            }
        }
        
        if let Err(e) = fs::create_dir_all(temp_dir) {
            eprintln!("{} Failed to create temp directory: {}", "❌".red(), e);
            std::process::exit(1);
        }
        
        processed_files.clear();
        
        for (idx, img_path) in image_files.iter().enumerate() {
            match image::open(img_path) {
                Ok(mut img) => {
                    // Apply enhancements
                    if let Some(b) = brightness {
                        if b != 1.0 {
                            img = enhancements::adjust_brightness(&img, b);
                        }
                    }
                    
                    if let Some(c) = contrast {
                        if c != 1.0 {
                            img = enhancements::adjust_contrast(&img, c);
                        }
                    }
                    
                    if flip_h {
                        img = enhancements::flip_horizontal(&img);
                    }
                    
                    if flip_v {
                        img = enhancements::flip_vertical(&img);
                    }
                    
                    if let Some(angle) = rotate {
                        use streambit_vision::enhancements::RotationAngle;
                        let rotation = match angle {
                            90 => RotationAngle::Rotate90,
                            180 => RotationAngle::Rotate180,
                            270 => RotationAngle::Rotate270,
                            _ => {
                                eprintln!("{} Invalid rotation angle: {}. Use 90, 180, or 270", "⚠️".yellow(), angle);
                                RotationAngle::Rotate90
                            }
                        };
                        img = enhancements::rotate(&img, rotation);
                    }
                    
                    if grayscale {
                        img = enhancements::to_grayscale(&img);
                    }
                    
                    // Apply filters
                    if let Some(sigma) = blur {
                        if sigma > 0.0 {
                            img = filters::gaussian_blur(&img, sigma);
                        }
                    }
                    
                    if sharpen {
                        img = filters::sharpen(&img);
                    }
                    
                    if edge_detect {
                        img = filters::edge_detection(&img);
                    }
                    
                    // Save filtered image
                    let filtered_path = format!("{}/filtered_{:04}.jpg", temp_dir, idx);
                    if let Err(e) = img.save(&filtered_path) {
                        eprintln!("{} Failed to save filtered image: {}", "⚠️".yellow(), e);
                    } else {
                        processed_files.push(filtered_path);
                    }
                }
                Err(e) => {
                    eprintln!("{} Failed to open {}: {}", "⚠️".yellow(), img_path, e);
                }
            }
        }
        
        println!("✅ Applied filters to {} images", processed_files.len().to_string().bright_green());
        println!("{}", "=".repeat(60).bright_black());
    }

    // Process images
    let start = Instant::now();
    let processor = ImageProcessor::new().with_resize_mode(resize_mode);

    match processor.load_batch(processed_files.clone(), Some((width, height)), None) {
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

fn process_audio(
    folder_path: &str,
    target_sample_rate: Option<u32>,
    to_mono: bool,
    do_normalize: bool,
    trim_threshold: Option<f32>,
    save_output: Option<&str>,
    format_str: &str,
    clean: bool,
) {
    use streambit_audio::{AudioLoader, AudioProcessor};
    
    println!("{}", "🎵 StreamBit Audio Processor".bright_cyan().bold());
    println!("{}", "=".repeat(60).bright_black());

    // Read folder
    println!("📂 Reading folder: {}", folder_path.bright_white());
    let mut audio_files = Vec::new();

    match fs::read_dir(folder_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if matches!(ext_str.as_str(), "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac") {
                        if let Some(path_str) = path.to_str() {
                            audio_files.push(path_str.to_string());
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

    if audio_files.is_empty() {
        eprintln!("{} No audio files found in folder", "❌".red());
        std::process::exit(1);
    }

    println!("✅ Found {} audio files", audio_files.len().to_string().bright_green());
    
    // Show processing options
    let mut options = Vec::new();
    if let Some(sr) = target_sample_rate {
        options.push(format!("Resample to {}Hz", sr));
    }
    if to_mono {
        options.push("Convert to Mono".to_string());
    }
    if do_normalize {
        options.push("Normalize".to_string());
    }
    if let Some(threshold) = trim_threshold {
        options.push(format!("Trim Silence (threshold: {})", threshold));
    }
    
    if !options.is_empty() {
        println!("🔧 Processing: {}", options.join(", ").bright_magenta());
    }
    
    if let Some(output_dir) = save_output {
        println!("💾 Output directory: {}", output_dir.bright_yellow());
        println!("📄 Output format: {}", format_str.to_uppercase().bright_cyan());
        
        // Clean if requested
        if clean && std::path::Path::new(output_dir).exists() {
            println!("🧹 Cleaning old files from: {}", output_dir.bright_yellow());
            if let Err(e) = fs::remove_dir_all(output_dir) {
                eprintln!("{} Failed to clean output directory: {}", "⚠️".yellow(), e);
            }
        }
        
        // Create output directory
        if let Err(e) = fs::create_dir_all(output_dir) {
            eprintln!("{} Failed to create output directory: {}", "❌".red(), e);
            std::process::exit(1);
        }
    }
    
    println!("{}", "=".repeat(60).bright_black());

    // Process audio files
    let start = Instant::now();
    let mut processed_count = 0;

    for (idx, audio_path) in audio_files.iter().enumerate() {
        print!("Processing {}/{}: {}... ", idx + 1, audio_files.len(), audio_path.bright_white());
        
        match AudioLoader::load(audio_path) {
            Ok(mut audio) => {
                // Apply processing
                if let Some(target_sr) = target_sample_rate {
                    if audio.sample_rate != target_sr {
                        match AudioProcessor::resample(&audio, target_sr) {
                            Ok(resampled) => audio = resampled,
                            Err(e) => {
                                eprintln!("{} Resample failed: {}", "❌".red(), e);
                                continue;
                            }
                        }
                    }
                }
                
                if to_mono && audio.channels > 1 {
                    match AudioProcessor::to_mono(&audio) {
                        Ok(mono) => audio = mono,
                        Err(e) => {
                            eprintln!("{} Mono conversion failed: {}", "❌".red(), e);
                            continue;
                        }
                    }
                }
                
                if do_normalize {
                    AudioProcessor::normalize(&mut audio);
                }
                
                if let Some(threshold) = trim_threshold {
                    match AudioProcessor::trim_silence(&audio, threshold) {
                        Ok(trimmed) => audio = trimmed,
                        Err(e) => {
                            eprintln!("{} Trim failed: {}", "❌".red(), e);
                            continue;
                        }
                    }
                }
                
                // Save if output directory specified
                if let Some(output_dir) = save_output {
                    let output_path = format!("{}/processed_{:04}.{}", output_dir, idx, format_str);
                    
                    // Save as WAV
                    use hound;
                    let spec = hound::WavSpec {
                        channels: audio.channels,
                        sample_rate: audio.sample_rate,
                        bits_per_sample: 16,
                        sample_format: hound::SampleFormat::Int,
                    };
                    
                    if let Ok(mut writer) = hound::WavWriter::create(&output_path, spec) {
                        for &sample in &audio.samples {
                            let sample_i16 = (sample * 32767.0).max(-32768.0).min(32767.0) as i16;
                            writer.write_sample(sample_i16).ok();
                        }
                        writer.finalize().ok();
                    }
                }
                
                println!("{}", "✓".green());
                processed_count += 1;
            }
            Err(e) => {
                eprintln!("{} Failed: {}", "❌".red(), e);
            }
        }
    }

    let duration = start.elapsed();
    let time_ms = duration.as_secs_f64() * 1000.0;
    let throughput = processed_count as f64 / duration.as_secs_f64();

    println!("{}", "=".repeat(60).bright_black());
    println!("{}", "✅ Processing Complete!".bright_green().bold());
    println!("{}", "=".repeat(60).bright_black());
    println!("📊 {} Processed: {}", "Files".bright_white(), processed_count.to_string().bright_cyan());
    println!("⏱️  {} {:.2} ms", "Time:".bright_white(), time_ms.to_string().bright_cyan());
    println!("🚀 {} {:.2} files/sec", "Throughput:".bright_white(), throughput.to_string().bright_green().bold());
    println!("{}", "=".repeat(60).bright_black());
}
