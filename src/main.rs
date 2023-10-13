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

#[derive(Debug, Deserialize, Clone)]
struct Template {
    template_name: String,
    image_path: String,
    font_path: String,
    text_fields: Vec<TextField>,
}

#[derive(Debug, Deserialize, Clone)]
struct TextField {
    //color: String,
    //max_size: u32,
    default_text: String,
    x_left: f32,
    y_top: f32,
    width: f32,
    height: f32,
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

fn render_text_simple(font: &Font) -> RgbImage {
    // Generate blank image canvas
    let width: u32 = 400;
    let height: u32 = 300;
    let mut image = RgbImage::new(width, height);

    // Set up layout struct and styling options
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.reset(&LayoutSettings {
        max_height: Some(height as f32),
        max_width: Some(width as f32),
        horizontal_align: HorizontalAlign::Center,
        vertical_align: VerticalAlign::Middle,
        wrap_style: WrapStyle::Word,
        ..Default::default()
    });

    // Add text to layout
    let fonts_ref = &[font];
    layout.append(fonts_ref, &TextStyle::new("Text rendering is ", 32.0, 0));
    layout.append(fonts_ref, &TextStyle::new("fucking ", 48.0, 0));
    layout.append(fonts_ref, &TextStyle::new("difficult!", 32.0, 0));

    // Generate glyph pattern from the lyout
    let glyphs = layout.glyphs();
    for glyph in glyphs.iter() {
        // Generate pixel layout for each glyph
        let (metrics, bytes) = font.rasterize_config(glyph.key);

        // Print pixels to the image canvas
        for x in 0..metrics.width {
            for y in 0..metrics.height {
                let mask = bytes[x + y * metrics.width];
                let pixel = Rgb([mask, mask, mask]);
                let x = x as u32 + glyph.x as u32;
                let y = y as u32 + glyph.y as u32;
                image.put_pixel(x, y, pixel);
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

#[get("/test")]
async fn template_test(fonts: web::Data<HashMap<String, Font>>) -> impl Responder {
    let font_name = "fonts/BebasNeue-Regular.ttf".to_string();
    serve_image_to_client(render_text_simple(fonts.get(&font_name).unwrap()))
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
            let rendered_image = template_image;
            serve_image_to_client(rendered_image)
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
            .service(template_test)
            .service(template_default)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
