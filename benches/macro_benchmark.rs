use criterion::{criterion_group, criterion_main, Criterion};
use moore_neighborhood::moore;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("macro d=2, r=2", |b| b.iter(|| moore!(2, 2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
