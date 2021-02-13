use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prime_factors::alt_implementations;
use alt_implementations::every_iteration;
use alt_implementations::odd_iteration;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("every_iteration", |b| b.iter(|| every_iteration::factors(black_box(93_819_012_551))));
    c.bench_function("odd_iteration", |b| b.iter(||  odd_iteration::factors(black_box(93_819_012_551))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);