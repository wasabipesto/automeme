//! Benchmarks for the main crate

use automeme::{add_text_to_image, get_template_resources, load_templates};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("load all templates from disk", |b| {
        b.iter(|| load_templates())
    });

    let templates = load_templates();
    let template = templates.get("weatherboy").unwrap();
    c.bench_function("load resources from disk for single template", |b| {
        b.iter(|| get_template_resources(template))
    });

    let template = get_template_resources(template);
    let image = template.image;
    c.bench_function("render text in all fields", |b| {
        b.iter(|| {
            for text_field in &template.text_fields {
                add_text_to_image(&text_field, image.clone(), &template.font);
            }
        })
    });

    let templates = load_templates();
    c.bench_function("load and render everything", |b| {
        b.iter(|| {
            let template = templates.get("weatherboy").unwrap();
            let template = get_template_resources(template);
            let mut image = template.image;
            for text_field in &template.text_fields {
                image = add_text_to_image(&text_field, image, &template.font);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
