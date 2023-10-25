//! Benchmarks for the main crate

use automeme::{get_template_resources, load_templates};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("load templates from json", |b| b.iter(|| load_templates()));

    let templates = load_templates();
    let template = templates.get("pikachu").unwrap();
    c.bench_function("load resources from template", |b| {
        b.iter(|| get_template_resources(template))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
