use criterion::{black_box, criterion_group, criterion_main, Criterion};
use moore_neighborhood::dynamic;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ported d=3, r=1", |b| {
        b.iter(|| dynamic::moore(black_box(1), black_box(3)))
    });
    c.bench_function("ported d=2, r=1", |b| {
        b.iter(|| dynamic::moore(black_box(1), black_box(2)))
    });
    c.bench_function("ported d=2, r=2", |b| {
        b.iter(|| dynamic::moore(black_box(2), black_box(2)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
