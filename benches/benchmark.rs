use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jp_deinflector::deinflect;

fn benchmark_deinflect(c: &mut Criterion) {
    let mut group = c.benchmark_group("deinflection");

    // Simple case
    group.bench_function("single inflection", |b| {
        b.iter(|| deinflect(black_box("食べた")))
    });

    // Normal case
    group.bench_function("normal number of inflections", |b| {
        b.iter(|| deinflect(black_box("言ってなかった")))
    });

    // Complex case
    group.bench_function("many inflections", |b| {
        b.iter(|| deinflect(black_box("食べさせられたくなかった")))
    });

    group.finish();
}

criterion_group!(benches, benchmark_deinflect);
criterion_main!(benches);
