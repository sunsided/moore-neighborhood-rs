use criterion::{black_box, criterion_group, criterion_main, Criterion};
use moore_neighborhood::generic_dimension;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ported generic d=3, r=1", |b| {
        b.iter(|| generic_dimension::moore::<3>(black_box(1)))
    });
    c.bench_function("ported generic d=2, r=1", |b| {
        b.iter(|| generic_dimension::moore::<2>(black_box(1)))
    });
    c.bench_function("ported generic d=2, r=2", |b| {
        b.iter(|| generic_dimension::moore::<2>(black_box(2)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
