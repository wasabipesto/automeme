use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use image::{Rgb, RgbImage, DynamicImage};
use rand::Rng;
use std::collections::HashMap;
use std::io::{Cursor, Seek};
use serde::Deserialize;
use serde_json;
use glob::glob;

#[derive(Debug, Deserialize, Clone)]
struct TemplateSimple {
    template_name: String,
    image_filename: String,
    text_fields: Vec<TextField>,
}

#[derive(Debug, Clone)]
struct Template {
    image: DynamicImage,
    text_fields: Vec<TextField>,
}

#[derive(Debug, Deserialize, Clone)]
struct TextField {
    //font: String,
    //color: String,
    //size: u32,
    default_text: String,
    x_start: u32,
    y_start: u32,
    x_space: u32,
    y_space: u32,
}

fn load_templates() -> HashMap<String, Template> {
    glob("templates/*/*.json")
        .expect("Failed to read glob pattern")
        .filter_map(|entry| entry.ok())
        .map(|file_path| {
            let json_content = std::fs::read_to_string(&file_path).expect("Failed to read file");
            let template: TemplateSimple = serde_json::from_str(&json_content).expect("Failed to deserialize JSON");
            let image_path = format!("templates/{}", template.image_filename);
            let image = image::open(image_path).expect("Failed to open image file");
            (template.template_name.clone(), Template { image, text_fields: template.text_fields })
        })
        .collect()
}

fn generate_random_noise_image() -> DynamicImage {
    let mut rng = rand::thread_rng();

    let width = 400;
    let height = 300;
    let mut image = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = Rgb([rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255)]);
            image.put_pixel(x, y, pixel);
        }
    }

    DynamicImage::ImageRgb8(image)
}

fn serve_image_to_client(image: DynamicImage) -> HttpResponse {
    let mut png_data = Cursor::new(Vec::new());
    image
        .write_to(&mut png_data, image::ImageOutputFormat::Png)
        .expect("Failed to write PNG data");

    png_data.seek(std::io::SeekFrom::Start(0)).expect("Failed to seek in PNG data");

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
            serve_image_to_client(template.image.clone())
        }
        None => {
            HttpResponse::NotFound().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting...");
    let templates = load_templates();
    println!("Loaded {} templates.", templates.len());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(templates.clone()))
            .service(index)
            .service(template_test)
            .service(template_default)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}