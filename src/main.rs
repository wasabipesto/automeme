//! Automeme generates memes and serves them over HTTP in a human-friendly way.
//! URLs are designed to be easily type-able to predictably generate the
//! desired image, and then fetched by e.g. a chatroom's link preview service.

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::unused_async)]
#![allow(clippy::needless_pass_by_value)]

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use core::u8;
use fontdue::layout::{
    CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle,
};
use fontdue::{Font, FontSettings};
use glob::glob;
use image::{Rgb, RgbImage};
use maud::{html, Markup};
use rand::seq::IteratorRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::{metadata, File};
use std::io::{Cursor, Read, Result, Seek, SeekFrom};

const FONT_GEOMETRY_SCALE: f32 = 60.0;
const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

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

/// The full version of the template with loaded data.
#[derive(Debug, Clone)]
struct TemplateFull {
    // template_name: String,
    image: RgbImage,
    font: Font,
    text_fields: Vec<TextField>,
}

/// Each text field represents a location where text can be rendered. Text will
/// be shrunk until it fits in the field specified.
#[derive(Debug, Deserialize, Clone)]
struct TextField {
    /// The text that goes in each field
    text: String,
    /// Whether the text should be forced into uppercase
    uppercase: bool,
    /// Distance from the top-left, in [x, y] pixels, where the text field begins
    start: [f32; 2],
    /// Distance from the top-left, in [x, y] pixels, where the text field ends
    end: [f32; 2],
    /// Default size of the text in this field
    text_size: f32,
    /// Color of the text in RGB
    text_color: [u8; 3],
    /// Color of the text border in RGB
    border_color: [u8; 3],
}

/// Load all resources necessary for server startup and check that all
/// referenced files exist.
fn load_templates() -> HashMap<String, Template> {
    println!("Server starting...");

    // Load and deserialize all JSON files in the templates directory.
    let templates: HashMap<String, Template> = glob("templates/*.json")
        .expect("Failed to resolve glob pattern")
        .filter_map(std::result::Result::ok)
        .map(|file_path| {
            let json_content =
                std::fs::read_to_string(file_path).expect("Failed to read JSON file");
            let template: Template =
                serde_json::from_str(&json_content).expect("Failed to deserialize JSON");
            (template.template_name.clone(), template)
        })
        .collect();

    // Check all referenced files exist
    for template in templates.values() {
        assert!(
            metadata(&template.image_path).is_ok(),
            "Could not find file {}",
            &template.image_path
        );
        assert!(
            metadata(&template.image_path).is_ok(),
            "Could not find file {}",
            &template.image_path
        );
    }

    println!("Loaded {} templates.", templates.len());
    templates
}

/// Given a Template, return a tuple of that Template plus the associated image
/// and font data. Panics if the image or font could not be found since they
/// should have been checked at startup.
fn get_template_resources(template: &Template) -> TemplateFull {
    TemplateFull {
        //template_name: template.template_name.clone(),
        image: match image::open(&template.image_path) {
            Ok(image) => image.to_rgb8(),
            Err(e) => panic!("Could not open file {} {}", &template.image_path, e),
        },
        font: match File::open(&template.font_path) {
            Ok(mut font_file) => {
                let mut font_bytes = Vec::new();
                font_file
                    .read_to_end(&mut font_bytes)
                    .expect("Failed to read font data");
                Font::from_bytes(
                    font_bytes,
                    FontSettings {
                        collection_index: 0,
                        scale: FONT_GEOMETRY_SCALE,
                    },
                )
                .expect("Failed to load font data")
            }
            Err(e) => panic!("Could not open file {} {}", &template.image_path, e),
        },
        text_fields: template.text_fields.clone(),
    }
}

/// Given a template name, get all assciated data. Returns None if the template
/// was not found. Returns a random template if "random" is used.
fn get_template_data(
    template_name: String,
    templates: &web::Data<HashMap<String, Template>>,
) -> Option<TemplateFull> {
    println!("Serving template {}", &template_name);

    // Special case - random
    if template_name == "random" {
        let (_, template) = templates.iter().choose(&mut rand::thread_rng()).unwrap();
        return Some(get_template_resources(template));
    }

    // Find matching template
    templates
        .get(&template_name)
        .map(get_template_resources)
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
        x: text_field.start[0],
        y: text_field.start[1],
        max_height: Some(text_field.end[1] - text_field.start[1]),
        max_width: Some(text_field.end[0] - text_field.start[0]),
        horizontal_align: HorizontalAlign::Center,
        vertical_align: VerticalAlign::Middle,
        wrap_style: WrapStyle::Word,
        ..Default::default()
    });
    layout
}

/// Renders text onto an image for one field.
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
fn add_text_to_image(text_field: &TextField, mut image: RgbImage, font: &Font) -> RgbImage {
    let mut layout = get_field_text_layout(text_field);

    // Set interior & border color
    let pixel_interior: Rgb<u8> = Rgb(text_field.text_color);
    let pixel_border: Rgb<u8> = Rgb(text_field.border_color);

    // Optionally convert to uppercase
    let text = if text_field.uppercase {
        text_field.text.to_uppercase()
    } else {
        text_field.text.clone()
    };

    // Add text to layout
    let mut text_size = text_field.text_size;
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

/// Index of all templates with a little help text.
#[get("/")]
async fn template_index(templates: web::Data<HashMap<String, Template>>) -> Result<Markup> {
    Ok(html! {
        html {
            head {
                title { "😂 automeme" }
            }
            body style="margin:20px;" {
                h1 { "😂 automeme" }
                p {
                    "Automeme generates memes and serves them over HTTP in a human-friendly way. URLs are designed to be easily type-able to predictably generate the desired image, and then fetched by e.g. a chatroom's link preview service."
                }
                p {
                    "To get an image with the default text, simply fetch the image by template name from /{template-name}. For instance, you can get the surprised pikachu meme from " a href="pikachu" { "/pikachu" } " or the \"Wouldn't you like to know, weather boy?\" meme from " a href="weatherboy" { "/weatherboy" } "."
                }
                p {
                    "If you want to edit the text of a meme, or add text to a meme with no default text, you can use the " strong { "/f" } " or " strong { "/s" } " options. The " strong { "/f " } " option allows you to overwrite the text of a meme to your own, like adding \"mfw code doesn't compile\" to the surprised pikachu template. To do this, take the default image path like " a href="pikachu" { "/pikachu" } " and add /f/{your-text} to make " a href="pikachu/f/mfw-code-doesn't-compile" { "/pikachu/f/mfw-code-doesn't-compile" } ". The " strong { "/s" } " option replaces existing text in the template to your own with the pattern /s/{old-text}/{new-text}, allowing you to quickly turn \"Wouldn't you like to know, weather boy?\" into " a href="weatherboy/s/weather-boy/type-checker" { "\"Wouldn't you like to know, type checker?\"" } " For memes with multiple fields, use | to move to the next field. Spaces are substituted from both - and _."
                }
                @for template in templates.values() {
                    a href=(template.template_name) {
                        img
                            src=(template.template_name)
                            title=(template.template_name)
                            style="max-height:250px; max-width:300px; margin:20px;"
                            {}
                    }
                }
                p {
                    "You can find the source for this project at " a href="https://github.com/wasabipesto/automeme" { "https://github.com/wasabipesto/automeme" } "."
                }
            }
        }
    })
}

/// Renders all templates with lorem ipsum text for bounds testing.
#[get("/lorem")]
async fn template_index_lorem(templates: web::Data<HashMap<String, Template>>) -> Result<Markup> {
    Ok(html! {
        html {
            head {
                title { "😂 automeme" }
            }
            body style="margin:20px;" {
                p {
                    a href=("/") { "Back to normal index." }
                }
                @for template in templates.values() {
                    @let path = format!("{}/l", template.template_name);
                    a href=(path) {
                        img
                            src=(path)
                            title=(template.template_name)
                            style="max-height:350px; max-width:400px; margin:20px;"
                            {}
                    }
                }
            }
        }
    })
}

/// Finds a template by name and renders it with default settings.
#[get("/{template_name}")]
async fn template_default(
    path: web::Path<String>,
    templates: web::Data<HashMap<String, Template>>,
) -> impl Responder {
    let template_name = path.into_inner();
    match get_template_data(template_name, &templates) {
        Some(template) => {
            let mut image = template.image;
            for text_field in template.text_fields {
                image = add_text_to_image(&text_field, image, &template.font);
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
) -> impl Responder {
    let (template_name, full_text) = path.into_inner();
    match get_template_data(template_name, &templates) {
        Some(template) => {
            let mut image = template.image;
            let text_fields = override_text_fields(
                template.text_fields,
                clean_text_to_vec(path_to_clean_text(full_text)),
            );
            for text_field in text_fields {
                image = add_text_to_image(&text_field, image, &template.font);
            }
            serve_image_to_client(&image)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

/// Renders a template with lorem ipsum text.
#[get("/{template_name}/l")]
async fn template_lorem(
    path: web::Path<String>,
    templates: web::Data<HashMap<String, Template>>,
) -> impl Responder {
    let template_name = path.into_inner();
    match get_template_data(template_name, &templates) {
        Some(template) => {
            let mut image = template.image;
            let lorem_vec = vec![String::from(LOREM_IPSUM); template.text_fields.len()];
            let text_fields = override_text_fields(template.text_fields, lorem_vec);
            for text_field in text_fields {
                image = add_text_to_image(&text_field, image, &template.font);
            }
            serve_image_to_client(&image)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

/// Renders a template by replacing text via a simple pattern.
#[get("/{template_name}/s/{old_text}/{new_text}")]
async fn template_sed(
    path: web::Path<(String, String, String)>,
    templates: web::Data<HashMap<String, Template>>,
) -> impl Responder {
    let (template_name, old_text, new_text) = path.into_inner();
    match get_template_data(template_name, &templates) {
        Some(template) => {
            let mut image = template.image;
            let text_fields = regex_text_fields(
                template.text_fields,
                path_to_clean_text(old_text),
                path_to_clean_text(new_text),
            );
            for text_field in text_fields {
                image = add_text_to_image(&text_field, image, &template.font);
            }
            serve_image_to_client(&image)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

/// Server startup tasks.
#[actix_web::main]
async fn main() -> Result<()> {
    // Start the server
    let templates = load_templates();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(templates.clone()))
            .service(template_index)
            .service(template_index_lorem)
            .service(template_default)
            .service(template_fulltext)
            .service(template_lorem)
            .service(template_sed)
    })
    .bind(env::var("HTTP_BIND").unwrap_or(String::from("0.0.0.0:8888")))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_template_index() {
        let templates = load_templates();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(templates.clone()))
                .service(template_index)
                .service(template_index_lorem)
                .service(template_default)
                .service(template_fulltext)
                .service(template_lorem)
                .service(template_sed),
        )
        .await;
        let req = test::TestRequest::default().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_template_pikachu_default() {
        let templates = load_templates();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(templates.clone()))
                .service(template_index)
                .service(template_index_lorem)
                .service(template_default)
                .service(template_fulltext)
                .service(template_lorem)
                .service(template_sed),
        )
        .await;
        let req = test::TestRequest::default().uri("/pikachu").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_template_pikachu_fulltext() {
        let templates = load_templates();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(templates.clone()))
                .service(template_index)
                .service(template_index_lorem)
                .service(template_default)
                .service(template_fulltext)
                .service(template_lorem)
                .service(template_sed),
        )
        .await;
        let req = test::TestRequest::default()
            .uri("/pikachu/f/test")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
