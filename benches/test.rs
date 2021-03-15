use std::fmt::Display;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn print_generic<T: Display>(num: T) {
    println!("number: {}", num);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("the test", |b| b.iter(|| print_generic("test")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);