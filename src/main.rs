#[macro_use]
extern crate rocket;
use image::{Rgb, RgbImage};
use rocket::fs::NamedFile;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/red")]
async fn red() -> Option<NamedFile> {
    let fractal_image = generate_image();
    let temp_file = "image.png";
    fractal_image.save(temp_file).ok()?;
    NamedFile::open(temp_file).await.ok()
}

fn generate_image() -> RgbImage {
    let mut img = RgbImage::new(200, 200);
    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([255, 0, 0]);
    }
    img
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, red])
}
