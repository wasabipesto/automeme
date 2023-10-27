//! Benchmarks for the main crate

use automeme::{
    get_template_from_disk, get_template_names, startup_check_all_resources,
    startup_load_all_resources,
};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("get names from template files", |b| {
        b.iter(|| get_template_names())
    });
    c.bench_function("load one template and resources from disk", |b| {
        b.iter(|| get_template_from_disk(&"weatherboy".to_owned()))
    });
    c.bench_function("load all templates and validate all resources", |b| {
        b.iter(|| startup_check_all_resources())
    });
    c.bench_function("load all templates and all resources into memory", |b| {
        b.iter(|| startup_load_all_resources())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
