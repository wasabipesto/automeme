//! Benchmarks for the main crate

use automeme_core::{
    get_template_from_disk, get_template_names, render_template, startup_check_all_resources,
    startup_load_all_resources,
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub fn standard_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("automeme-core");
    group
        .sample_size(100)
        .noise_threshold(0.05)
        .warm_up_time(Duration::new(2, 0))
        .measurement_time(Duration::new(10, 0));

    group.bench_function("get names from template files", |b| {
        b.iter(|| get_template_names().unwrap())
    });

    group.bench_function("load one template and resources from disk", |b| {
        b.iter(|| {
            get_template_from_disk(&"weatherboy".to_owned())
                .unwrap()
                .unwrap()
        })
    });

    group.bench_function("load all templates and validate all resources", |b| {
        b.iter(|| startup_check_all_resources().unwrap())
    });

    let template = get_template_from_disk(&"weatherboy".to_owned())
        .unwrap()
        .unwrap();
    group.bench_function("render a loaded template", |b| {
        b.iter(|| render_template(template.clone()))
    });

    group.bench_function("load and render a template by name", |b| {
        b.iter(|| {
            let template = get_template_from_disk(&"weatherboy".to_owned())
                .unwrap()
                .unwrap();
            render_template(template);
        })
    });

    group.finish();
}

pub fn long_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("automeme-core");
    group
        .sample_size(10)
        .noise_threshold(0.05)
        .warm_up_time(Duration::new(2, 0))
        .measurement_time(Duration::new(30, 0));

    group.bench_function("load all templates and all resources into memory", |b| {
        b.iter(|| startup_load_all_resources())
    });

    group.finish();
}

criterion_group!(benches, standard_benches, long_benches);
criterion_main!(benches);
