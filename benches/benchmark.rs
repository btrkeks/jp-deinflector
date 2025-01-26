use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jp_deinflector::deinflect;

fn benchmark_deinflect(c: &mut Criterion) {
    let mut group = c.benchmark_group("deinflection");

    // Simple case
    group.bench_function("simple verb", |b| {
        b.iter(|| deinflect(black_box("食べた")))
    });

    // Complex case
    group.bench_function("complex inflection chain", |b| {
        b.iter(|| deinflect(black_box("食べさせられなかった")))
    });

    group.finish();
}

criterion_group!(benches, benchmark_deinflect);
criterion_main!(benches);