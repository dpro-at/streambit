use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use futures_util::TryStreamExt;
use serde::Serialize;
use std::io::Write;
use std::time::Instant;
use streambit_vision::{ImageProcessor, ResizeMode};

#[derive(Serialize)]
struct ProcessResult {
    success: bool,
    message: String,
    images_processed: usize,
    time_ms: f64,
    throughput: f64,
    shapes: Vec<Vec<usize>>,
    python_time_ms: Option<f64>,
    python_throughput: Option<f64>,
    speedup: Option<f64>,
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

async fn process_images(mut payload: Multipart) -> Result<HttpResponse> {
    let mut temp_files = Vec::new();
    let start = Instant::now();

    // Save uploaded files temporarily
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .unwrap_or("unknown")
            .to_string();

        let filepath = format!("temp_{}", filename);
        let mut f = std::fs::File::create(&filepath)?;

        while let Some(chunk) = field.try_next().await? {
            f.write_all(&chunk)?;
        }

        temp_files.push(filepath);
    }

    if temp_files.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ProcessResult {
            success: false,
            message: "No images uploaded".to_string(),
            images_processed: 0,
            time_ms: 0.0,
            throughput: 0.0,
            shapes: vec![],
            python_time_ms: None,
            python_throughput: None,
            speedup: None,
        }));
    }

    // Process images
    let processor = ImageProcessor::new().with_resize_mode(ResizeMode::Bilinear);

    match processor.load_batch(temp_files.clone(), Some((224, 224)), None) {
        Ok(batch) => {
            let duration = start.elapsed();
            let time_ms = duration.as_secs_f64() * 1000.0;
            let throughput = batch.len() as f64 / duration.as_secs_f64();

            let shapes: Vec<Vec<usize>> = batch
                .tensors()
                .iter()
                .map(|t| t.shape().to_vec())
                .collect();

            // Run Python benchmark for comparison
            let (python_time_ms, python_throughput, speedup) = run_python_benchmark(&temp_files);

            // Cleanup temp files
            for file in &temp_files {
                std::fs::remove_file(file).ok();
            }

            Ok(HttpResponse::Ok().json(ProcessResult {
                success: true,
                message: format!("Successfully processed {} images", batch.len()),
                images_processed: batch.len(),
                time_ms,
                throughput,
                shapes,
                python_time_ms,
                python_throughput,
                speedup,
            }))
        }
        Err(e) => {
            // Cleanup temp files
            for file in &temp_files {
                std::fs::remove_file(file).ok();
            }

            Ok(HttpResponse::InternalServerError().json(ProcessResult {
                success: false,
                message: format!("Error processing images: {}", e),
                images_processed: 0,
                time_ms: 0.0,
                throughput: 0.0,
                shapes: vec![],
                python_time_ms: None,
                python_throughput: None,
                speedup: None,
            }))
        }
    }
}

#[derive(serde::Deserialize)]
struct FolderRequest {
    paths: Vec<String>,
}

async fn process_folder(req: web::Json<FolderRequest>) -> Result<HttpResponse> {
    use std::fs;
    
    let start = Instant::now();
    let folder_paths = &req.paths;
    
    // Read all image files from all folders
    let mut image_files = Vec::new();
    
    for folder_path in folder_paths {
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
                // Log error but continue with other folders if possible? 
                // For now, let's just log it and continue
                println!("Failed to read folder {}: {}", folder_path, e);
            }
        }
    }
    
    if image_files.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ProcessResult {
            success: false,
            message: "No images found in the selected folders".to_string(),
            images_processed: 0,
            time_ms: 0.0,
            throughput: 0.0,
            shapes: vec![],
            python_time_ms: None,
            python_throughput: None,
            speedup: None,
        }));
    }
    
    // Process images
    let processor = ImageProcessor::new().with_resize_mode(ResizeMode::Bilinear);
    
    match processor.load_batch(image_files.clone(), Some((224, 224)), None) {
        Ok(batch) => {
            let duration = start.elapsed();
            let time_ms = duration.as_secs_f64() * 1000.0;
            let throughput = batch.len() as f64 / duration.as_secs_f64();

            let shapes: Vec<Vec<usize>> = batch
                .tensors()
                .iter()
                .map(|t| t.shape().to_vec())
                .collect();

            // Run Python benchmark for comparison
            let (python_time_ms, python_throughput, speedup) = run_python_benchmark(&image_files);

            Ok(HttpResponse::Ok().json(ProcessResult {
                success: true,
                message: format!("Successfully processed {} images from {} folders", batch.len(), folder_paths.len()),
                images_processed: batch.len(),
                time_ms,
                throughput,
                shapes,
                python_time_ms,
                python_throughput,
                speedup,
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ProcessResult {
                success: false,
                message: format!("Error processing images: {}", e),
                images_processed: 0,
                time_ms: 0.0,
                throughput: 0.0,
                shapes: vec![],
                python_time_ms: None,
                python_throughput: None,
                speedup: None,
            }))
        }
    }
}

fn run_python_benchmark(files: &[String]) -> (Option<f64>, Option<f64>, Option<f64>) {
    use std::process::Command;
    // use std::time::Duration;

    // Skip Python benchmark for large batches (too slow)
    if files.len() > 100 {
        println!("Skipping Python benchmark for {} images (too many)", files.len());
        return (None, None, None);
    }

    // Try different Python commands
    let python_commands = vec![
        "python",
        "python3",
        r"C:\Users\Dpro GmbH\AppData\Local\Programs\Python\Python312\python.exe",
    ];

    for python_cmd in python_commands {
        let output = Command::new(python_cmd)
            .arg("streambit-web-ui/benchmark_python.py")
            .args(files)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Ok(result) = serde_json::from_str::<serde_json::Value>(&stdout) {
                    let py_time = result["time_ms"].as_f64();
                    let py_throughput = result["throughput"].as_f64();
                    
                    if let (Some(py_t), Some(py_tp)) = (py_time, py_throughput) {
                        return (Some(py_t), Some(py_tp), None);
                    }
                }
            }
            _ => continue,
        }
    }
    
    // If Python is not available, return None (comparison will be skipped)
    (None, None, None)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 StreamBit Web UI Starting...");
    println!("📡 Server running at: http://localhost:8080");
    println!("🌐 Open your browser and navigate to the URL above");
    println!("\nPress Ctrl+C to stop the server\n");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/process", web::post().to(process_images))
            .route("/process-folder", web::post().to(process_folder))
            .service(Files::new("/static", "./streambit-web-ui/static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
