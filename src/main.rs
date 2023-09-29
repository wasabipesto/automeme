use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use image::{Rgb, RgbImage};
use rand::Rng;
use std::collections::HashMap;
use std::io::{Cursor, Seek};
use serde::{Deserialize, Serialize};
use serde_json;
use glob::glob;


#[derive(Debug, Deserialize, Serialize, Clone)]
struct Template {
    template_name: String,
    //image: RgbImage,
    text_fields: Vec<TextField>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TextField {
    font: String,
    color: String,
    size: u32,
    default_text: String,
    bound_x_left: u32,
    bound_x_right: u32,
    bound_y_bottom: u32,
    bound_y_top: u32,
}

fn load_templates() -> HashMap<String, Template> {
    let templates: HashMap<String, Template> = glob("templates/*.json")
        .expect("Failed to read glob pattern")
        .filter_map(|entry| entry.ok())
        .map(|file_path| {
            let json_content = std::fs::read_to_string(&file_path).expect("Failed to read file");
            let template: Template = serde_json::from_str(&json_content).expect("Failed to deserialize JSON");
            (template.template_name.clone(), template)
        })
        .collect();

    println!("Loaded {} templates", templates.len());
    templates
}

fn generate_random_noise_image() -> RgbImage {
    let mut rng = rand::thread_rng();

    let width = 400;
    let height = 300;
    let mut img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = Rgb([rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255)]);
            img.put_pixel(x, y, pixel);
        }
    }

    img
}

fn serve_image_to_client(image: RgbImage) -> HttpResponse {
    // Create buffer with cursor so we can stream the data in chunks
    let mut png_data = Cursor::new(Vec::new());

    // Transfer the image data to the buffer
    image
        .write_to(&mut png_data, image::ImageOutputFormat::Png)
        .expect("Failed to write PNG data");

    // Move the cursor back to the beginning of the data to ensure it can be read
    png_data.seek(std::io::SeekFrom::Start(0)).expect("Failed to seek in PNG data");

    // Prepare the response data for the client
    HttpResponse::Ok()
        .content_type("image/png")
        .body(png_data.into_inner())
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[get("/test")]
async fn template_test() -> impl Responder {
    let image = generate_random_noise_image();
    serve_image_to_client(image)
}

#[get("/{template_name}")]
async fn template_default(
    template_name: web::Path<String>,
    templates: web::Data<HashMap<String, Template>>,
) -> impl Responder {
    
    let template_name = template_name.to_string();
    match templates.get(&template_name) {
        Some(template) => {
            // Do something with the template
            // For example, return a response with the template data
            HttpResponse::Ok().json(template)
        }
        None => {
            // Handle the case when the template is not found
            HttpResponse::NotFound().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load templates once during application setup
    let templates = load_templates();

    HttpServer::new(move || {
        App::new()
            // Configure state with the loaded templates
            .app_data(web::Data::new(templates.clone()))
            .service(index)
            .service(template_test)
            .service(template_default)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}