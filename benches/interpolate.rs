use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use template_cke::Interpolator;

const FORMAT: &str = "potato{0}-{2}/${4:03}_{*},{-1}suffix";
const CHUNK_IDX: &[u64] = &[0, 1, 2, 3, 4, 5, 6];

fn bench_build(c: &mut Criterion) {
    c.bench_function("build", |b| {
        b.iter(|| Interpolator::try_new(black_box(FORMAT), Some(":")))
    });
}

fn bench_interpolate(c: &mut Criterion) {
    let interp = Interpolator::try_new(FORMAT, Some(":")).unwrap();
    c.bench_function("interpolate", |b| {
        b.iter(|| interp.interpolate(black_box(CHUNK_IDX)))
    });
}

criterion_group!(benches, bench_build, bench_interpolate);
criterion_main!(benches);
