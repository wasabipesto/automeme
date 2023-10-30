//! Automeme generates memes and serves them over HTTP in a human-friendly way.
//! URLs are designed to be easily type-able to predictably generate the
//! desired image, and then fetched by e.g. a chatroom's link preview service.
//! This is the image loading and generation portion of the crate.

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::unused_async)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::must_use_candidate)]

use core::u8;
use fontdue::layout::{
    CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle,
};
use fontdue::{Font, FontSettings};
use glob::glob;
use image::{Pixel, Rgba, RgbaImage};
// use rand::seq::IteratorRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::{metadata, File};
use std::io::Read;
use std::path::PathBuf;

const FONT_GEOMETRY_SCALE: f32 = 60.0;

/// Data from the JSON template files. This struct is only used to deserialize the
/// files before the full template struct is built.
#[derive(Debug, Deserialize, Clone)]
pub struct TemplateJSON {
    /// The name of the template as referenced in urls and lookup keys
    pub template_name: String,
    /// The relative path of the base image from the project root, also used as a lookup key
    pub image_path: String,
    /// The relative path of the font from the project root, also used as a lookup key
    pub font_path: String,
    /// All places text can go in an image
    pub text_fields: Vec<TextField>,
}

/// The full version of the template with loaded data. This can be directly
/// passed to the render fuction to get a completed image.
#[derive(Debug, Clone)]
pub struct Template {
    // template_name: String,
    pub image: RgbaImage,
    pub font: Font,
    pub text_fields: Vec<TextField>,
}

/// Each text field represents a location where text can be rendered. Text will
/// be shrunk until it fits in the field specified. If the text field extends
/// past the bounds of the image, an error will be thrown at runtime.
#[derive(Debug, Deserialize, Clone)]
pub struct TextField {
    /// The default text that goes in each field
    pub text: String,
    /// Whether the text should be forced into uppercase
    pub uppercase: bool,
    /// Distance from the top-left, in [x, y] pixels, where the text field begins
    pub start: [u32; 2],
    /// Distance from the top-left, in [x, y] pixels, where the text field ends
    pub end: [u32; 2],
    /// Default size of the text in this field, may be reduced to fit in bounds
    pub text_size: f32,
    /// Color of the text in RGB
    pub text_color: [u8; 3],
    /// Color of the text border in RGB (optional)
    pub border_color: Option<[u8; 3]>,
    /// Color of the text shadow in RGB (optional)
    pub shadow_color: Option<[u8; 3]>,
}

/// Get a list of templates based on json filenames.
///
/// # Errors
/// Will return Err if:
/// - the pattern glob cannot be expanded
/// - any json file in the templates directory has no filename
/// - any filename cannot be converted to a string
pub fn get_template_names() -> Result<Vec<String>, String> {
    let paths = glob("templates/*.json").map_err(|e| format!("Failed to expand glob: {e}"))?;
    let names: Result<Vec<String>, String> = paths
        .map(|path| {
            let file_path = path.map_err(|e| format!("Failed to read file path: {e}"))?;
            let file_stem = file_path.file_stem().ok_or("Failed to get file stem")?;
            let file_str = file_stem
                .to_str()
                .ok_or("Failed to convert OsStr to string")?;
            Ok(file_str.to_string())
        })
        .collect();

    names
}

/// Load a selected json file from the disk. Returns None if no template by
/// that name exists.
///
/// # Errors
/// Will return Err if the json string cannot be read from the file or the
/// struct cannot be deserialized from the json string.
fn get_json_from_disk(template_name: &String) -> Result<Option<TemplateJSON>, String> {
    let file_path = PathBuf::from(format!("templates/{template_name}.json"));
    if !file_path.exists() {
        return Ok(None);
    }
    let json_string = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read JSON file: {e}"))?;
    let template_json: TemplateJSON = serde_json::from_str(&json_string)
        .map_err(|e| format!("Failed to deserialize JSON: {e}"))?;

    Ok(Some(template_json))
}

/// Load a selected template and all resources from the disk. Returns None if
/// no template by that name exists.
///
/// # Errors
/// Will return Err if:
/// - `get_json_from_disk` returns Err for the given template name
/// - Image file cannot be opened or decoded
/// - Font file cannot be opened, read, or loaded.
pub fn get_template_from_disk(template_name: &String) -> Result<Option<Template>, String> {
    if let Some(template_json) = get_json_from_disk(template_name)? {
        // Successfully located and read the json file
        // Open and decode the image
        let image = image::open(&template_json.image_path)
            .map_err(|e| format!("Failed to open file {}: {}", &template_json.image_path, e))
            .map(|image| image.to_rgba8())
            .map_err(|e| {
                format!(
                    "Failed to decode image {}: {}",
                    &template_json.image_path, e
                )
            })?;
        // Open and decode font
        let font = File::open(&template_json.font_path)
            .map_err(|e| format!("Failed to open file {}: {}", &template_json.font_path, e))
            .and_then(|mut font_file| {
                let mut font_bytes = Vec::new();
                font_file
                    .read_to_end(&mut font_bytes)
                    .map_err(|e| format!("Failed to read font data: {e}"))?;
                Font::from_bytes(
                    font_bytes,
                    FontSettings {
                        collection_index: 0,
                        scale: FONT_GEOMETRY_SCALE,
                    },
                )
                .map_err(|e| format!("Failed to load font data: {e}"))
            })?;
        // Get text fields
        let text_fields = template_json.text_fields;

        // Return the template
        Ok(Some(Template {
            image,
            font,
            text_fields,
        }))
    } else {
        // Could not find a template by that name
        Ok(None)
    }
}

/// Load each template file in the templates directory and check that all of
/// the referenced files (fonts, images) exist.
///
/// # Errors
/// Will return Err if:
/// - `get_template_names` returns Err
/// - `get_template_from_disk` returns Err or None when loading a template by name
/// - any file referenced by a template does not exist
pub fn startup_check_all_resources() -> Result<usize, String> {
    // Get list of template names from glob
    let template_names = get_template_names()?;

    // Load json for each template
    let templates_json: Result<Vec<TemplateJSON>, String> = template_names
        .iter()
        .map(|name| match get_json_from_disk(name)? {
            Some(template) => Ok(template),
            None => Err(format!("Error: Template '{name}' not found.")),
        })
        .collect();

    // Check all referenced files exist
    for template in templates_json.clone()? {
        for file_path in [template.image_path, template.font_path] {
            metadata(&file_path).map_err(|e| format!("Failed to read file {file_path}: {e}"))?;
        }
    }

    Ok(templates_json?.len())
}

/// Load each template file in the templates directory and load everything into
/// a single `HashMap` for faster lookup.
///
/// # Errors
/// Will return Err if:
/// - `get_template_names` returns Err
/// - `get_template_from_disk` returns Err or None when loading a template by name
pub fn startup_load_all_resources() -> Result<HashMap<String, Template>, String> {
    // Get list of template names from glob
    let template_names = get_template_names()?;

    // Load all resources for each template
    let templates: Result<HashMap<String, Template>, String> = template_names
        .iter()
        .map(|name| match get_template_from_disk(name)? {
            Some(template) => Ok((name.clone(), template)),
            None => Err(format!("Error: Template '{name}' not found.")),
        })
        .collect();

    templates
}

/// Create a transparent image layer with the rendered text to be overlayed on
/// the final image. To generate a border layer, increase the `blot_radius` to
/// something greater than 0.
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn generate_text_layer(
    layout: &Layout,
    font: &Font,
    text_color: [u8; 3],
    blot_radius: f32,
) -> RgbaImage {
    // Generate mask canvas
    let mut text_canvas = RgbaImage::new(
        layout.settings().max_width.unwrap_or_default() as u32,
        layout.settings().max_height.unwrap_or_default() as u32,
    );

    // Generate blot pattern
    let mut blot_pattern = Vec::new();
    for theta in 0..360 {
        let theta_rad = theta as f32 * PI / 180.0;
        let point = (
            (blot_radius * theta_rad.cos()) as i32,
            (blot_radius * theta_rad.sin()) as i32,
        );
        if !blot_pattern.contains(&point) {
            blot_pattern.push(point);
        }
    }

    // Generate glyph pattern from the layout
    for glyph in layout.glyphs() {
        // Generate pixel layout for each glyph
        let (metrics, bytes) = font.rasterize_config(glyph.key);
        let glyph_start = (glyph.x as u32, glyph.y as u32);

        // Print pixels to the canvas
        for x in 0..metrics.width {
            for y in 0..metrics.height {
                // Get coverage data from rasterization
                let byte_index = y * metrics.width + x;
                if let Some(mask) = bytes.get(byte_index) {
                    // Blot pixels around the rendered pixel
                    for blot_pattern_point in &blot_pattern {
                        let blot_point = (
                            (glyph_start.0 + x as u32).saturating_add_signed(blot_pattern_point.0),
                            (glyph_start.1 + y as u32).saturating_add_signed(blot_pattern_point.1),
                        );
                        if let Some(p) =
                            text_canvas.get_pixel_mut_checked(blot_point.0, blot_point.1)
                        {
                            p.blend(&Rgba([text_color[0], text_color[1], text_color[2], *mask]));
                        }
                    }
                }
            }
        }
    }

    text_canvas
}

/// Overlay a text layer with transparency onto the base image. To simulate a
/// drop shadow, increase the starting position to move the canvas down/right.
pub fn blend_layer_onto_image(image: &mut RgbaImage, layer: &RgbaImage, start: (u32, u32)) {
    for (x, y, overlay_pixel) in layer.enumerate_pixels() {
        if overlay_pixel.0[3] != 0 {
            if let Some(p) = image.get_pixel_mut_checked(start.0 + x, start.1 + y) {
                p.blend(overlay_pixel);
            };
        }
    }
}

/// Given one text field, generates all text effects and layers them onto the
/// base image.
//#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
fn render_text_field_on_image(
    mut image: RgbaImage,
    text_field: &TextField,
    font: &Font,
) -> RgbaImage {
    // Get field width & height
    let field_width = (text_field.end[0] - text_field.start[0]) as f32;
    let field_height = (text_field.end[1] - text_field.start[1]) as f32;

    // Generate a text field layout object
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.reset(&LayoutSettings {
        x: 0.0,
        y: 0.0,
        max_height: Some(field_height),
        max_width: Some(field_width),
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
    while layout.height() > field_height {
        text_size -= 1.0;
        layout.clear();
        layout.append(&[font], &TextStyle::new(&text, text_size, 0));
    }

    // Generate text layer
    let text_canvas = generate_text_layer(&layout, font, text_field.text_color, 0.0);

    // Generate & add shadow layer
    if let Some(shadow_color) = text_field.shadow_color {
        let shadow_offset = (text_size * 0.06) as i32;
        let shadow_canvas = generate_text_layer(&layout, font, shadow_color, 0.0);
        blend_layer_onto_image(
            &mut image,
            &shadow_canvas,
            (
                text_field.start[0].saturating_add_signed(shadow_offset),
                text_field.start[1].saturating_add_signed(shadow_offset),
            ),
        );
    };

    // Generate & add border layer
    if let Some(border_color) = text_field.border_color {
        let border_size = text_size * 0.03;
        let border_canvas = generate_text_layer(&layout, font, border_color, border_size);
        blend_layer_onto_image(
            &mut image,
            &border_canvas,
            (text_field.start[0], text_field.start[1]),
        );
    };

    // Add text layer
    blend_layer_onto_image(
        &mut image,
        &text_canvas,
        (text_field.start[0], text_field.start[1]),
    );

    image
}

/// Renders a template completely. If a template is passed directly after
/// loading, all default values will be used. Edit the template before passing
/// here to e.g. edit the text in each field.
pub fn render_template(template: Template) -> RgbaImage {
    let mut image = template.image;
    for text_field in template.text_fields {
        image = render_text_field_on_image(image, &text_field, &template.font);
    }
    image
}
