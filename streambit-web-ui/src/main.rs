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

    // Skip Python benchmark for very large batches (too slow)
    // Increased to 5000 to allow benchmarking your 1865 images
    if files.len() > 5000 {
        println!("Skipping Python benchmark for {} images (too many)", files.len());
        return (None, None, None);
    }

    // Try different Python commands (prioritizing user's explicit path)
    let python_commands = vec![
        r"C:\Users\Dpro GmbH\AppData\Local\Programs\Python\Python312\python.exe",
        "python",
        "python3",
    ];

    let mut collected_errors = Vec::new();

    for python_cmd in python_commands {
        use std::process::Stdio;

        // Try to spawn the process (removed mut per warning)
        let child_process = Command::new(python_cmd)
            .arg("streambit-web-ui/benchmark_python.py")
            .arg("--json-stdin")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match child_process {
            Ok(mut child) => {
                // Write files JSON to stdin
                if let Some(mut stdin) = child.stdin.take() {
                    if let Ok(json_data) = serde_json::to_string(files) {
                        let _ = stdin.write_all(json_data.as_bytes());
                    }
                }

                // Wait for output
                match child.wait_with_output() {
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
                    Ok(output) => {
                        collected_errors.push(format!("Cmd '{}' failed with stderr: {}", python_cmd, String::from_utf8_lossy(&output.stderr)));
                    }
                    Err(e) => {
                        collected_errors.push(format!("Cmd '{}' failed to wait: {}", python_cmd, e));
                    }
                }
            }
            Err(e) => {
                collected_errors.push(format!("Cmd '{}' failed to start: {}", python_cmd, e));
            }
        }
    }
    
    // If we're here, all failed. Print debug errors.
    println!("⚠️  Python benchmark skipped (all commands failed):");
    for err in collected_errors {
        println!("  - {}", err);
    }
    
    // Return None (comparison will be skipped)
    (None, None, None)
}

#[derive(serde::Deserialize)]
struct DownloadRequest {
    limit: usize,
    dataset: String,
}

#[derive(serde::Serialize)]
struct DownloadResult {
    success: bool,
    message: String,
    path: Option<String>,
}

async fn download_dataset(req: web::Json<DownloadRequest>) -> Result<HttpResponse> {
    use std::process::Command;
    
    // Path to the python script
    let script_path = "benchmarks/download_dataset.py";
    
    // We need to run this with the user's python
    // For simplicity, we'll use the same logic as benchmark to find python
    // Ideally this should be a shared function
    let python_commands = vec![
        r"C:\Users\Dpro GmbH\AppData\Local\Programs\Python\Python312\python.exe",
        "python",
        "python3",
    ];

    let mut last_error = String::from("No python command tried yet");

    for python_cmd in python_commands {
        println!("Trying to run python script with: {}", python_cmd);
        
        let output = Command::new(python_cmd)
            .arg(script_path)
            .arg("--limit")
            .arg(req.limit.to_string())
            .arg("--dataset")
            .arg(&req.dataset)
            .output();

       match output {
           Ok(out) => {
               if out.status.success() {
                   return Ok(HttpResponse::Ok().json(DownloadResult {
                       success: true,
                       message: format!("Successfully downloaded {} images from '{}'", req.limit, req.dataset),
                       path: Some("benchmarks/data/images".to_string()),
                   }));
               } else {
                   let stderr = String::from_utf8_lossy(&out.stderr);
                   let stdout = String::from_utf8_lossy(&out.stdout);
                   last_error = format!("Status failure. Stderr: {}. Stdout: {}", stderr, stdout);
                   println!("Python command failed: {}", last_error);
               }
           }
           Err(e) => {
               last_error = format!("Failed to spawn command: {}", e);
               println!("Spawn error: {}", last_error);
           }
       }
    }

    Ok(HttpResponse::InternalServerError().json(DownloadResult {
        success: false,
        message: format!("Failed to run Python script. Details: {}", last_error),
        path: None,
    }))
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
            .route("/api/download-dataset", web::post().to(download_dataset))
            .service(Files::new("/static", "./streambit-web-ui/static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
