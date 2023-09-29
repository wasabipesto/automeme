#[macro_use]
extern crate rocket;
use image::{Rgb, RgbImage};
use rocket::fs::NamedFile;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test_image")]
async fn test_image() -> Option<NamedFile> {
    let image = generate_image();
    let filename = "image.png";
    image.save(filename).ok()?;
    NamedFile::open(filename).await.ok()
}

fn generate_image() -> RgbImage {
    let mut image = RgbImage::new(200, 200);
    for (_, _, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgb([255, 0, 0]);
    }
    image
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, test_image])
}
