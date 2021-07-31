use criterion::{criterion_group, criterion_main, Criterion};
use moore_neighborhood::generic_full;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ported full generic d=3, r=1", |b| {
        b.iter(|| generic_full::moore::<1, 3, 26>())
    });
    c.bench_function("ported full generic d=2, r=1", |b| {
        b.iter(|| generic_full::moore::<1, 2, 8>())
    });
    c.bench_function("ported full generic d=2, r=2", |b| {
        b.iter(|| generic_full::moore::<2, 2, 24>())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
