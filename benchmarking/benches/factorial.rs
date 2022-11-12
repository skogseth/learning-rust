use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};

use benchmarking::factorial;

pub fn benchmark_variants(c: &mut Criterion) {
    let mut group = c.benchmark_group("Factorial");
    for i in [10u128, 20u128, 30u128].iter() {
        group.bench_with_input(BenchmarkId::new("Iter", i), i, |b, i| b.iter(|| factorial::iter(*i)));
        group.bench_with_input(BenchmarkId::new("Looping", i), i, |b, i| b.iter(|| factorial::looping(*i)));
        group.bench_with_input(BenchmarkId::new("Recursive", i), i, |b, i| b.iter(|| factorial::recursive(*i)));
        group.bench_with_input(BenchmarkId::new("Recursive (with tail)", i), i, |b, i| b.iter(|| factorial::recursive_with_tail(*i)));
    }
    group.finish()
}

criterion_group!(benches, benchmark_variants);
criterion_main!(benches);

