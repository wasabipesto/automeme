//! Automeme generates memes and serves them over HTTP in a human-friendly way.
//! URLs are designed to be easily type-able to predictably generate the
//! desired image, and then fetched by e.g. a chatroom's link preview service.
//! This is the image loading and generation portion of the crate.

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::unused_async)]
#![allow(clippy::needless_pass_by_value)]

use core::cmp::max;
use core::u8;
use fontdue::layout::{
    CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle,
};
use fontdue::{Font, FontSettings};
use glob::glob;
use image::{Rgb, RgbImage, Rgba, RgbaImage};
use rand::seq::IteratorRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::{metadata, File};

const FONT_GEOMETRY_SCALE: f32 = 60.0;

/// Data from the JSON template files. At startup these are loaded in and then the
/// image and font paths are checked and loaded as well.
#[derive(Debug, Deserialize, Clone)]
pub struct Template {
    /// The name of the template as referenced in urls and lookup keys
    pub template_name: String,
    /// The relative path of the base image from the project root, also used as a lookup key
    pub image_path: String,
    /// The relative path of the font from the project root, also used as a lookup key
    pub font_path: String,
    /// All places text can go in an image
    pub text_fields: Vec<TextField>,
}

/// The full version of the template with loaded data.
#[derive(Debug, Clone)]
pub struct TemplateFull {
    // template_name: String,
    pub image: RgbImage,
    pub font: Font,
    pub text_fields: Vec<TextField>,
}

/// Each text field represents a location where text can be rendered. Text will
/// be shrunk until it fits in the field specified.
#[derive(Debug, Deserialize, Clone)]
pub struct TextField {
    /// The text that goes in each field
    pub text: String,
    /// Whether the text should be forced into uppercase
    pub uppercase: bool,
    /// Distance from the top-left, in [x, y] pixels, where the text field begins
    pub start: [u32; 2],
    /// Distance from the top-left, in [x, y] pixels, where the text field ends
    pub end: [u32; 2],
    /// Default size of the text in this field
    pub text_size: f32,
    /// Color of the text in RGB
    pub text_color: [u8; 3],
    /// Color of the text border in RGB (optional)
    pub border_color: Option<[u8; 3]>,
    /// Color of the text shadow in RGB (optional)
    pub shadow_color: Option<[u8; 3]>,
}

/// Load all resources necessary for server startup and check that all
/// referenced files exist.
pub fn load_templates() -> HashMap<String, Template> {
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

    //println!("Loaded {} templates.", templates.len());
    templates
}

/// Given a Template, return a tuple of that Template plus the associated image
/// and font data. Panics if the image or font could not be found since they
/// should have been checked at startup.
pub fn get_template_resources(template: &Template) -> TemplateFull {
    TemplateFull {
        //template_name: template.template_name.clone(),
        image: match image::open(&template.image_path) {
            Ok(image) => image.to_rgb8(),
            Err(e) => panic!("Could not open file {} {}", &template.image_path, e),
        },
        font: match File::open(&template.font_path) {
            Ok(mut font_file) => {
                let mut font_bytes = Vec::new();
                std::io::Read::read_to_end(&mut font_file, &mut font_bytes)
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
pub fn get_template_data(
    template_name: String,
    templates: &HashMap<String, Template>,
) -> Option<TemplateFull> {
    println!("Serving template {}", &template_name);

    // Special case - random
    if template_name == "random" {
        let (_, template) = templates.iter().choose(&mut rand::thread_rng()).unwrap();
        return Some(get_template_resources(template));
    }

    // Find matching template
    templates.get(&template_name).map(get_template_resources)
}

/// Create a transparent image layer with the rendered text.
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_possible_truncation)]
pub fn generate_text_canvas(
    layout: &Layout,
    font: &Font,
    width: u32,
    height: u32,
    text_color: [u8; 3],
    blot_radius: f32,
) -> RgbaImage {
    // Generate mask canvas
    let mut text_canvas = RgbaImage::new(width, height);

    // Generate blot pattern
    let mut blot_pattern = Vec::new();
    for theta in 0..360 {
        let theta_rad = theta as f32 * PI / 180.0;
        let point = (
            (blot_radius * theta_rad.cos()) as i64,
            (blot_radius * theta_rad.sin()) as i64,
        );
        if !blot_pattern.contains(&point) {
            blot_pattern.push(point);
        }
    }

    // Generate glyph pattern from the layout
    for glyph in layout.glyphs() {
        // Generate pixel layout for each glyph
        let (metrics, bytes) = font.rasterize_config(glyph.key);

        // Print pixels to the canvas
        for x in 0..metrics.width {
            for y in 0..metrics.height {
                // Get coverage data from rasterization
                let byte_index = y * metrics.width + x;
                let mask = bytes.get(byte_index).expect("Failed to get glyph data!");

                // Blot pixels around the rendered pixel
                for blot_pattern_point in &blot_pattern {
                    let blot_point = (
                        (glyph.x as i64 + x as i64 + blot_pattern_point.0) as u32,
                        (glyph.y as i64 + y as i64 + blot_pattern_point.1) as u32,
                    );
                    if let Some(current_pixel) =
                        text_canvas.get_pixel_checked(blot_point.0, blot_point.1)
                    {
                        let current_mask = current_pixel[3];
                        let new_mask = max(current_mask, *mask);
                        text_canvas.put_pixel(
                            blot_point.0,
                            blot_point.1,
                            Rgba([text_color[0], text_color[1], text_color[2], new_mask]),
                        );
                    } else {
                        println!("Pixel ({},{}) out of bounds!", blot_point.0, blot_point.1);
                    }
                }
            }
        }
    }

    text_canvas
}

/// Overlay a text layer with transparency onto the image.
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn blend_layer_on_image(
    image: &mut RgbImage,
    text_canvas: &RgbaImage,
    start_pos: (u32, u32),
    offset: (i64, i64),
) {
    // Check bounds fit on image
    if i64::from(start_pos.0) + offset.0 < 0
        || i64::from(start_pos.0) + offset.0 + i64::from(text_canvas.width())
            > i64::from(image.width())
        || i64::from(start_pos.1) + offset.1 < 0
        || i64::from(start_pos.1) + offset.1 + i64::from(text_canvas.height())
            > i64::from(image.height())
    {
        panic!("Text field exceeds image bounds!")
    }

    // Iterate over text canvas
    for x in 0..text_canvas.width() {
        for y in 0..text_canvas.height() {
            // Get canvas data
            let overlay_pixel = text_canvas.get_pixel_checked(x, y).unwrap();
            let overlay_alpha = f32::from(overlay_pixel.0[3]) / 255.0;

            // Skip if nothing to write
            if overlay_alpha == 0.0 {
                continue;
            }

            // Get background data
            let bg_pixel_loc = (
                (i64::from(x) + i64::from(start_pos.0) + offset.0) as u32,
                (i64::from(y) + i64::from(start_pos.1) + offset.1) as u32,
            );
            let bg_pixel = image.get_pixel(bg_pixel_loc.0, bg_pixel_loc.1);

            // Blend the colors
            let blended_pixel = Rgb([
                ((1.0 - overlay_alpha) * f32::from(bg_pixel.0[0])
                    + overlay_alpha * f32::from(overlay_pixel.0[0])) as u8,
                ((1.0 - overlay_alpha) * f32::from(bg_pixel.0[1])
                    + overlay_alpha * f32::from(overlay_pixel.0[1])) as u8,
                ((1.0 - overlay_alpha) * f32::from(bg_pixel.0[2])
                    + overlay_alpha * f32::from(overlay_pixel.0[2])) as u8,
            ]);

            // Save to image
            image.put_pixel(
                x + start_pos.0 + offset.0 as u32,
                y + start_pos.1 + offset.1 as u32,
                blended_pixel,
            );
        }
    }
}

/// Renders text onto an image for one field.
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn add_text_to_image(text_field: &TextField, mut image: RgbImage, font: &Font) -> RgbImage {
    // Get field width & height
    let field_width = text_field.end[0] - text_field.start[0];
    let field_height = text_field.end[1] - text_field.start[1];

    // Generate a text field layout object
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.reset(&LayoutSettings {
        x: 0.0,
        y: 0.0,
        max_height: Some(field_height as f32),
        max_width: Some(field_width as f32),
        horizontal_align: HorizontalAlign::Center,
        vertical_align: VerticalAlign::Middle,
        wrap_style: WrapStyle::Word,
        ..Default::default()
    });

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
    while layout.height() > field_height as f32 {
        text_size -= 1.0;
        layout.clear();
        layout.append(&[font], &TextStyle::new(&text, text_size, 0));
    }

    // Generate text layer
    let text_canvas = generate_text_canvas(
        &layout,
        font,
        field_width,
        field_height,
        text_field.text_color,
        0.0,
    );

    // Generate & add shadow layer
    if let Some(shadow_color) = text_field.shadow_color {
        let shadow_offset = (text_size * 0.06) as i64;
        let shadow_canvas =
            generate_text_canvas(&layout, font, field_width, field_height, shadow_color, 0.0);
        blend_layer_on_image(
            &mut image,
            &shadow_canvas,
            (text_field.start[0], text_field.start[1]),
            (shadow_offset, shadow_offset),
        );
    };

    // Generate & add border layer
    if let Some(border_color) = text_field.border_color {
        let border_size = text_size * 0.03;
        let border_canvas = generate_text_canvas(
            &layout,
            font,
            field_width,
            field_height,
            border_color,
            border_size,
        );
        blend_layer_on_image(
            &mut image,
            &border_canvas,
            (text_field.start[0], text_field.start[1]),
            (0, 0),
        );
    };

    // Add text layer
    blend_layer_on_image(
        &mut image,
        &text_canvas,
        (text_field.start[0], text_field.start[1]),
        (0, 0),
    );

    image
}
