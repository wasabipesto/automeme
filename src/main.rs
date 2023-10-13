use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use fontdue;
use fontdue::layout::{
    CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle,
};
use fontdue::{Font, FontSettings};
use glob::glob;
use image::{Rgb, RgbImage};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::{Cursor, Seek};
use std::u8;

#[derive(Debug, Deserialize, Clone)]
struct Template {
    /// The name of the template as referenced in urls and lookup keys
    template_name: String,
    /// The relative path of the base image from the project root, also used as a lookup key
    image_path: String,
    /// The relative path of the font from the project root, also used as a lookup key
    font_path: String,
    /// All places text can go in an image
    text_fields: Vec<TextField>,
}

#[derive(Debug, Deserialize, Clone)]
struct TextField {
    /// The text that goes in each field
    text: String,
    /// Distance from the left, in pixels, where the text field begins
    xmin: f32,
    /// Distance from the bottom, in pixels, where the text field begins
    ymin: f32,
    /// Width of the field in pixels
    width: f32,
    /// Height of the field in pixels
    height: f32,
    /// Maximum size of the text in this field
    max_size: f32,
    /// Whether the text should be forced into uppercase
    uppercase: bool,
    // Color of the text in RGB
    //color: ,
}

fn load_templates() -> HashMap<String, Template> {
    glob("templates/*.json")
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|file_path| {
            let json_content =
                std::fs::read_to_string(&file_path).expect("Failed to read JSON file");
            let template: Template =
                serde_json::from_str(&json_content).expect("Failed to deserialize JSON");
            (template.template_name.clone(), template)
        })
        .collect()
}

fn load_images(templates: &HashMap<String, Template>) -> HashMap<String, RgbImage> {
    templates
        .iter()
        .map(|(_, template)| {
            (
                template.image_path.clone(),
                image::open(&template.image_path)
                    .expect("Failed to open image file")
                    .to_rgb8(),
            )
        })
        .collect()
}

fn load_fonts(templates: &HashMap<String, Template>) -> HashMap<String, Font> {
    templates
        .iter()
        .map(|(_, template)| {
            let mut font_bytes = Vec::new();
            File::open(&template.font_path)
                .and_then(|mut font_file| font_file.read_to_end(&mut font_bytes))
                .expect("Failed to read font file");
            let font_data = Font::from_bytes(font_bytes, FontSettings::default())
                .expect("Failed to load font data");
            (template.font_path.clone(), font_data)
        })
        .collect()
}

fn get_template_data(
    template_name: web::Path<String>,
    templates: web::Data<HashMap<String, Template>>,
    images: web::Data<HashMap<String, RgbImage>>,
    fonts: web::Data<HashMap<String, Font>>,
) -> Option<(Template, RgbImage, Font)> {
    let template_name = template_name.to_string();
    match templates.get(&template_name) {
        Some(template) => Some((
            template.clone(),
            images.get(&template.image_path).unwrap().clone(),
            fonts.get(&template.font_path).unwrap().clone(),
        )),
        None => None,
    }
}

fn add_text_to_image(text_field: &TextField, mut image: RgbImage, font: &Font) -> RgbImage {
    // Set up layout struct and styling options
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.reset(&LayoutSettings {
        x: text_field.xmin,
        y: text_field.ymin,
        max_height: Some(text_field.height),
        max_width: Some(text_field.width),
        horizontal_align: HorizontalAlign::Center,
        vertical_align: VerticalAlign::Middle,
        wrap_style: WrapStyle::Word,
        ..Default::default()
    });

    // Set color and fill threshold
    let pixel = Rgb([255, 255, 255]);
    let mask_cutoff = u8::MAX;

    // Optionally convert to uppercase
    let text = match text_field.uppercase {
        false => text_field.text.clone(),
        true => text_field.text.to_uppercase(),
    };

    // Add text to layout
    let mut text_size = text_field.max_size;
    layout.append(&[font], &TextStyle::new(&text, text_size, 0));

    // Shrink text to fit the field if necessary
    while layout.height() > layout.settings().max_height.unwrap() {
        text_size -= 1.0;
        layout.clear();
        layout.append(&[font], &TextStyle::new(&text, text_size, 0));
    }

    // Generate glyph pattern from the lyout
    for glyph in layout.glyphs().iter() {
        // Generate pixel layout for each glyph
        let (metrics, bytes) = font.rasterize_config(glyph.key);

        // Print pixels to the image canvas
        for x in 0..metrics.width {
            for y in 0..metrics.height {
                let byte_index = y * glyph.width + x;
                if bytes[byte_index] >= mask_cutoff {
                    let x = glyph.x as u32 + x as u32;
                    let y = glyph.y as u32 + y as u32;
                    image.put_pixel(x, y, pixel);
                }
            }
        }
    }

    image
}

fn serve_image_to_client(image: RgbImage) -> HttpResponse {
    let mut png_data = Cursor::new(Vec::new());
    image
        .write_to(&mut png_data, image::ImageOutputFormat::Png)
        .expect("Failed to write PNG data");

    png_data
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek in PNG data");

    HttpResponse::Ok()
        .content_type("image/png")
        .body(png_data.into_inner())
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[get("/{template_name}")]
async fn template_default(
    template_name: web::Path<String>,
    templates: web::Data<HashMap<String, Template>>,
    images: web::Data<HashMap<String, RgbImage>>,
    fonts: web::Data<HashMap<String, Font>>,
) -> impl Responder {
    match get_template_data(template_name, templates, images, fonts) {
        Some((template, template_image, font)) => {
            let mut image = template_image.clone();
            for text_field in template.text_fields {
                image = add_text_to_image(&text_field, image, &font);
            }
            serve_image_to_client(image)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting...");
    let templates = load_templates();
    println!("Loaded {} templates.", templates.len());
    let images = load_images(&templates);
    println!("Loaded {} images.", images.len());
    let fonts = load_fonts(&templates);
    println!("Loaded {} fonts.", fonts.len());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(templates.clone()))
            .app_data(web::Data::new(images.clone()))
            .app_data(web::Data::new(fonts.clone()))
            .service(index)
            .service(template_default)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
