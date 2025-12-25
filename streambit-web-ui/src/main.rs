use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
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
}

#[derive(Deserialize)]
struct ProcessConfig {
    resize_width: Option<u32>,
    resize_height: Option<u32>,
    resize_mode: Option<String>,
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
            }))
        }
    }
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
            .service(Files::new("/static", "./streambit-web-ui/static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
