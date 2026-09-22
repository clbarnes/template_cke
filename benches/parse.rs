use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use template_cke::parser::parse_parts;

const FORMAT: &str = "potato{0}-{2}/${4:03}_{*},{-1}suffix";

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    group.bench_function("handrolled", |b| {
        b.iter(|| black_box(parse_parts(black_box(FORMAT))))
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
