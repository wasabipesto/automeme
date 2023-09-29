use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use image::{Rgb, RgbImage};
use rand::Rng;
use std::io::{Cursor, Seek};

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
async fn template_default(template_name: web::Path<String>) -> impl Responder {
    let template_name = template_name.to_string();
    template_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // earlier entries get priority
            .service(index)
            .service(template_test)
            .service(template_default)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
