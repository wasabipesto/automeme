//! Automeme generates memes and serves them over HTTP in a human-friendly way.
//! This is the HTTP server portion of the crate.

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use automeme::{
    get_template_from_disk, get_template_names, render_template, startup_check_all_resources,
    Template, TextField,
};
use image::RgbaImage;
use maud::{html, Markup};
use std::env;
use std::io::{Cursor, Result, Seek, SeekFrom};

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

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

/// Streams the image data to the client and tells them it's a PNG file.
fn serve_image_to_client(image: &RgbaImage) -> HttpResponse {
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
async fn template_index() -> Result<Markup> {
    let template_list = get_template_names().unwrap();
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
                @for template_name in template_list {
                    a href=(template_name) {
                        img
                            src=(template_name)
                            title=(template_name)
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
async fn template_index_lorem() -> Result<Markup> {
    let template_list = get_template_names().unwrap();
    Ok(html! {
        html {
            head {
                title { "😂 automeme" }
            }
            body style="margin:20px;" {
                p {
                    a href=("/") { "Back to normal index." }
                }
                @for template_name in template_list {
                    @let path = format!("{}/l", template_name);
                    a href=(path) {
                        img
                            src=(path)
                            title=(template_name)
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
async fn template_default(path: web::Path<String>) -> impl Responder {
    let template_name = path.into_inner();
    match get_template_from_disk(&template_name).unwrap() {
        Some(template) => {
            println!("Serving template {} as default", &template_name);
            let image = render_template(template);
            serve_image_to_client(&image)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

/// Renders a template with entirely user-given text.
#[get("/{template_name}/f/{full_text}")]
async fn template_fulltext(path: web::Path<(String, String)>) -> impl Responder {
    let (template_name, full_text) = path.into_inner();
    match get_template_from_disk(&template_name).unwrap() {
        Some(template) => {
            println!("Serving template {} with fulltext", &template_name);
            let text_fields = override_text_fields(
                template.text_fields,
                clean_text_to_vec(path_to_clean_text(full_text)),
            );
            let image = render_template(Template {
                text_fields,
                ..template
            });
            serve_image_to_client(&image)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

/// Renders a template with lorem ipsum text.
#[get("/{template_name}/l")]
async fn template_lorem(path: web::Path<String>) -> impl Responder {
    let template_name = path.into_inner();
    match get_template_from_disk(&template_name).unwrap() {
        Some(template) => {
            println!("Serving template {} with lorem", &template_name);
            let lorem_vec = vec![String::from(LOREM_IPSUM); template.text_fields.len()];
            let text_fields = override_text_fields(template.text_fields, lorem_vec);
            let image = render_template(Template {
                text_fields,
                ..template
            });
            serve_image_to_client(&image)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

/// Renders a template by replacing text via a simple pattern.
#[get("/{template_name}/s/{old_text}/{new_text}")]
async fn template_sed(path: web::Path<(String, String, String)>) -> impl Responder {
    let (template_name, old_text, new_text) = path.into_inner();
    match get_template_from_disk(&template_name).unwrap() {
        Some(template) => {
            println!("Serving template {} with sed", &template_name);
            let text_fields = regex_text_fields(
                template.text_fields,
                path_to_clean_text(old_text),
                path_to_clean_text(new_text),
            );
            let image = render_template(Template {
                text_fields,
                ..template
            });
            serve_image_to_client(&image)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

/// Server startup tasks.
#[actix_web::main]
async fn main() -> Result<()> {
    // Validate resources
    let num_templates = startup_check_all_resources().unwrap();
    println!("Server started: {} templates validated.", num_templates);
    // Start the server
    HttpServer::new(move || {
        App::new()
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
        let app = test::init_service(
            App::new()
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
        let app = test::init_service(
            App::new()
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
        let app = test::init_service(
            App::new()
                .service(template_index)
                .service(template_index_lorem)
                .service(template_default)
                .service(template_fulltext)
                .service(template_lorem)
                .service(template_sed),
        )
        .await;
        let req = test::TestRequest::default()
            .uri("/pikachu/f/a")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_template_pikachu_lorem() {
        let app = test::init_service(
            App::new()
                .service(template_index)
                .service(template_index_lorem)
                .service(template_default)
                .service(template_fulltext)
                .service(template_lorem)
                .service(template_sed),
        )
        .await;
        let req = test::TestRequest::default().uri("/pikachu/l").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    #[ignore]
    async fn test_templates_all_default() {
        let app = test::init_service(
            App::new()
                .service(template_index)
                .service(template_index_lorem)
                .service(template_default)
                .service(template_fulltext)
                .service(template_lorem)
                .service(template_sed),
        )
        .await;
        for template_name in get_template_names().unwrap() {
            let req = test::TestRequest::default()
                .uri(&format!("/{}", template_name))
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }
    }

    #[actix_web::test]
    #[ignore]
    async fn test_templates_all_fulltext() {
        let app = test::init_service(
            App::new()
                .service(template_index)
                .service(template_index_lorem)
                .service(template_default)
                .service(template_fulltext)
                .service(template_lorem)
                .service(template_sed),
        )
        .await;
        for template_name in get_template_names().unwrap() {
            let req = test::TestRequest::default()
                .uri(&format!("/{}/f/a", template_name))
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }
    }

    #[actix_web::test]
    #[ignore]
    async fn test_templates_all_lorem() {
        let app = test::init_service(
            App::new()
                .service(template_index)
                .service(template_index_lorem)
                .service(template_default)
                .service(template_fulltext)
                .service(template_lorem)
                .service(template_sed),
        )
        .await;
        for template_name in get_template_names().unwrap() {
            let req = test::TestRequest::default()
                .uri(&format!("/{}/l", template_name))
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }
    }
}
