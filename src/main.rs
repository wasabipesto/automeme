use actix_web::{get, App, HttpServer, HttpResponse};
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

#[get("/")]
async fn handle_index() -> HttpResponse {
    HttpResponse::Ok()
        .body("Hello world!")
}

#[get("/test")]
async fn handle_test_image() -> HttpResponse {
    let image = generate_random_noise_image();

    // Use a Cursor to wrap the Vec<u8> and provide the required Seek implementation.
    let mut png_data = Cursor::new(Vec::new());
    image
        .write_to(&mut png_data, image::ImageOutputFormat::Png)
        .expect("Failed to write PNG data");

    // Move the cursor back to the beginning of the data to ensure it can be read.
    png_data.seek(std::io::SeekFrom::Start(0)).expect("Failed to seek in PNG data");

    // Send data to the client.
    HttpResponse::Ok()
        .content_type("image/png")
        .body(png_data.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(handle_index).service(handle_test_image)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
