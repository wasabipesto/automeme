//! Automeme-cli is a simple CLI tool for making memes with automeme-core.

extern crate clap;
use clap::Parser;

use arboard::{Clipboard, ImageData};
use automeme_core::{get_template_from_disk, render_template};
use image::RgbaImage;
use std::borrow::Cow;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// The template to use
    #[arg(short, long, default_value = "random")]
    template_name: String,
}

fn save_image_to_clipboard(image: &RgbaImage) -> () {
    let mut buffer = Vec::new();
    for pixel in image.pixels() {
        buffer.extend_from_slice(&pixel.0);
    }
    let mut clipboard = Clipboard::new().unwrap();
    clipboard
        .set_image(ImageData {
            width: image.width() as usize,
            height: image.height() as usize,
            bytes: Cow::from(image.as_flat_samples().samples),
        })
        .unwrap();
}

fn main() {
    // parse args from command line
    let cli = Cli::parse();
    if let Some(template) = get_template_from_disk(&cli.template_name).unwrap() {
        let image = render_template(template);
        save_image_to_clipboard(&image);
    } else {
        println!("Template not found.");
    }
}
