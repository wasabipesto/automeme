//! Automeme generates memes and serves them over HTTP in a human-friendly way.
//! URLs are designed to be easily type-able to predictably generate the
//! desired image, and then fetched by e.g. a chatroom's link preview service.

#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use core::u8;
use fontdue::layout::{
    CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle,
};
use fontdue::{Font, FontSettings};
use glob::glob;
use image::{Rgb, RgbImage};
use rand::seq::IteratorRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read, Result, Seek, SeekFrom};

const FONT_GEOMETRY_SCALE: f32 = 60.0;

/// Data from the JSON template files. At startup these are loaded in and then the
/// image and font paths are checked and loaded as well.
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

/// Each text field represents a location where text can be rendered. Text will
/// be shrunk until it fits in the field specified.
#[derive(Debug, Deserialize, Clone)]
struct TextField {
    /// The text that goes in each field
    text: String,
    /// Distance from the left, in pixels, where the text field begins
    xmin: f32,
    /// Distance from the top, in pixels, where the text field begins
    ymin: f32,
    /// Width of the field in pixels
    width: f32,
    /// Height of the field in pixels
    height: f32,
    /// Maximum size of the text in this field
    max_size: f32,
    /// Whether the text should be forced into uppercase
    uppercase: bool,
    /// Color of the text in RGB
    color: [u8; 3],
    // Color of the text border in RGB
    //border_color: [u8; 3],
}

/// Load and deserialize all JSON files in the templates directory.
fn load_templates() -> HashMap<String, Template> {
    glob("templates/*.json")
        .expect("Failed to resolve glob pattern")
        .filter_map(std::result::Result::ok)
        .map(|file_path| {
            let json_content =
                std::fs::read_to_string(file_path).expect("Failed to read JSON file");
            let template: Template =
                serde_json::from_str(&json_content).expect("Failed to deserialize JSON");
            (template.template_name.clone(), template)
        })
        .collect()
}

/// Load all images referred to by templates and convert to RGB8.
fn load_images(templates: &HashMap<String, Template>) -> HashMap<String, RgbImage> {
    templates
        .iter()
        .map(|(_, template)| {
            let file = image::open(&template.image_path);
            match file {
                Ok(image) => (template.image_path.clone(), image.to_rgb8()),
                Err(e) => panic!("Could not open file {} {}", &template.image_path, e),
            }
        })
        .collect()
}

/// Load all fonts referred to by templates and parses them.
fn load_fonts(templates: &HashMap<String, Template>) -> HashMap<String, Font> {
    templates
        .iter()
        .map(|(_, template)| {
            let mut font_bytes = Vec::new();
            let file = File::open(&template.font_path)
                .and_then(|mut font_file| font_file.read_to_end(&mut font_bytes));
            match file {
                Ok(_) => (),
                Err(e) => panic!("Could not open file {} {}", &template.font_path, e),
            }
            let font_data = Font::from_bytes(
                font_bytes,
                FontSettings {
                    collection_index: 0,
                    scale: FONT_GEOMETRY_SCALE,
                },
            )
            .expect("Failed to load font data");
            (template.font_path.clone(), font_data)
        })
        .collect()
}

/// Given a Template, return a tuple of that Template plus the associated image
/// and font data. Panics if the image or font could not be found since they
/// should have been loaded at startup.
fn get_template_resources(
    template: &Template,
    images: &web::Data<HashMap<String, RgbImage>>,
    fonts: &web::Data<HashMap<String, Font>>,
) -> (Template, RgbImage, Font) {
    (
        template.clone(),
        images
            .get(&template.image_path)
            .expect("Failed to get cached image")
            .clone(),
        fonts
            .get(&template.font_path)
            .expect("Failed to get cached font")
            .clone(),
    )
}

/// Given a template name, get all assciated data. Returns None if the template
/// was not found. Returns a random template if "random" is used.
fn get_template_data(
    template_name: String,
    templates: web::Data<HashMap<String, Template>>,
    images: web::Data<HashMap<String, RgbImage>>,
    fonts: web::Data<HashMap<String, Font>>,
) -> Option<(Template, RgbImage, Font)> {
    // Special case - random
    if template_name == "random" {
        let (_, template) = templates.iter().choose(&mut rand::thread_rng()).unwrap();
        return Some(get_template_resources(template, &images, &fonts));
    }

    // Find matching template
    templates
        .get(&template_name)
        .map(|template| get_template_resources(template, &images, &fonts))
}

/// Cleans a path and turns it into usable text.
fn path_to_clean_text(text: String) -> String {
    text.replace(['-', '_'], " ")
}

/// Divides a text based on delimiters.
fn clean_text_to_vec(text: String) -> Vec<String> {
    text.split('|').map(|s| s.trim().to_string()).collect()
}

/// Replaces text in each field with the text in the override vec. Extra strings
/// are ignored.
fn override_text_fields(
    mut text_fields: Vec<TextField>,
    override_strings: Vec<String>,
) -> Vec<TextField> {
    for (index, override_str) in override_strings.into_iter().enumerate() {
        if let Some(text_field) = text_fields.get_mut(index) {
            text_field.text = override_str;
        }
    }

    text_fields
}

/// Replaces text in each field with the pattern.
fn regex_text_fields(
    text_fields: Vec<TextField>,
    old_text: String,
    new_text: String,
) -> Vec<TextField> {
    text_fields
        .into_iter()
        .map(|field| TextField {
            text: field.text.replace(&old_text, &new_text),
            ..field
        })
        .collect()
}

/// Generates a layout struct with options from the settings.
fn get_field_text_layout(text_field: &TextField) -> Layout {
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
    layout
}

/// Renders text onto an image for one field.
fn add_text_to_image(text_field: &TextField, mut image: RgbImage, font: &Font) -> RgbImage {
    let mut layout = get_field_text_layout(text_field);

    // Set interior & border color
    let pixel_interior: Rgb<u8> = Rgb(text_field.color);
    let pixel_border: Rgb<u8> = Rgb([0, 0, 0]);

    // Optionally convert to uppercase
    let text = if text_field.uppercase {
        text_field.text.to_uppercase()
    } else {
        text_field.text.clone()
    };

    // Add text to layout
    let mut text_size = text_field.max_size;
    layout.append(&[font], &TextStyle::new(&text, text_size, 0));

    // Shrink text to fit the field if necessary
    while layout.height() > layout.settings().max_height.expect("No max_height!") {
        text_size -= 1.0;
        layout.clear();
        layout.append(&[font], &TextStyle::new(&text, text_size, 0));
    }

    // Generate glyph pattern from the lyout
    for glyph in layout.glyphs() {
        // Generate pixel layout for each glyph
        let (metrics, bytes) = font.rasterize_config(glyph.key);

        // Print pixels to the image canvas
        for x in 0..metrics.width {
            for y in 0..metrics.height {
                let byte_index = y * glyph.width + x;
                let x = glyph.x as u32 + x as u32;
                let y = glyph.y as u32 + y as u32;
                match bytes.get(byte_index) {
                    Some(255) => image.put_pixel(x, y, pixel_interior),
                    Some(0) => (),
                    Some(_) => image.put_pixel(x, y, pixel_border), // very hacky border fix
                    None => panic!("Failed to get byte index"),
                }
            }
        }
    }

    image
}

/// Streams the image data to the client and tells them it's a PNG file.
fn serve_image_to_client(image: &RgbImage) -> HttpResponse {
    let mut png_data = Cursor::new(Vec::new());
    image
        .write_to(&mut png_data, image::ImageOutputFormat::Png)
        .expect("Failed to write PNG data");

    png_data
        .seek(SeekFrom::Start(0))
        .expect("Failed to seek in PNG data");

    HttpResponse::Ok()
        .content_type("image/png")
        .body(png_data.into_inner())
}

/// Basic hello world index. TODO: Make more informative.
#[get("/")]
async fn template_index() -> impl Responder {
    "Hello world!"
}

/// Finds a template by name and renders it with default settings.
#[get("/{template_name}")]
async fn template_default(
    path: web::Path<String>,
    templates: web::Data<HashMap<String, Template>>,
    images: web::Data<HashMap<String, RgbImage>>,
    fonts: web::Data<HashMap<String, Font>>,
) -> impl Responder {
    let template_name = path.into_inner();
    match get_template_data(template_name, templates, images, fonts) {
        Some((template, template_image, font)) => {
            let mut image = template_image.clone();
            for text_field in template.text_fields {
                image = add_text_to_image(&text_field, image, &font);
            }
            serve_image_to_client(&image)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

/// Renders a template with entirely user-given text.
#[get("/{template_name}/f/{full_text}")]
async fn template_fulltext(
    path: web::Path<(String, String)>,
    templates: web::Data<HashMap<String, Template>>,
    images: web::Data<HashMap<String, RgbImage>>,
    fonts: web::Data<HashMap<String, Font>>,
) -> impl Responder {
    let (template_name, full_text) = path.into_inner();
    match get_template_data(template_name, templates, images, fonts) {
        Some((template, template_image, font)) => {
            let mut image = template_image.clone();
            let text_fields = override_text_fields(
                template.text_fields,
                clean_text_to_vec(path_to_clean_text(full_text)),
            );
            for text_field in text_fields {
                image = add_text_to_image(&text_field, image, &font);
            }
            serve_image_to_client(&image)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

/// Renders a template with entirely user-given text.
#[get("/{template_name}/s/{old_text}/{new_text}")]
async fn template_sed(
    path: web::Path<(String, String, String)>,
    templates: web::Data<HashMap<String, Template>>,
    images: web::Data<HashMap<String, RgbImage>>,
    fonts: web::Data<HashMap<String, Font>>,
) -> impl Responder {
    let (template_name, old_text, new_text) = path.into_inner();
    match get_template_data(template_name, templates, images, fonts) {
        Some((template, template_image, font)) => {
            let mut image = template_image.clone();
            let text_fields = regex_text_fields(
                template.text_fields,
                path_to_clean_text(old_text),
                path_to_clean_text(new_text),
            );
            for text_field in text_fields {
                image = add_text_to_image(&text_field, image, &font);
            }
            serve_image_to_client(&image)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

/// Server startup tasks.
#[actix_web::main]
async fn main() -> Result<()> {
    // Load in all resources
    println!("Server starting...");
    let templates = load_templates();
    println!("Loaded {} templates.", templates.len());
    let images = load_images(&templates);
    println!("Loaded {} images.", images.len());
    let fonts = load_fonts(&templates);
    println!("Loaded {} fonts.", fonts.len());

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(templates.clone()))
            .app_data(web::Data::new(images.clone()))
            .app_data(web::Data::new(fonts.clone()))
            .service(template_index)
            .service(template_default)
            .service(template_fulltext)
            .service(template_sed)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
