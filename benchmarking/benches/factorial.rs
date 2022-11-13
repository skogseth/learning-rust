use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};

use benchmarking::factorial;

pub fn benchmark_variants(c: &mut Criterion) {
    let mut group = c.benchmark_group("Factorial");
    let tests: [u128; 13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 20];
    for i in tests.iter() {
        group.bench_with_input(BenchmarkId::new("Iter", i), i, |b, i| b.iter(|| factorial::iter(*i)));
        group.bench_with_input(BenchmarkId::new("Looping", i), i, |b, i| b.iter(|| factorial::looping(*i)));
        group.bench_with_input(BenchmarkId::new("Recursive", i), i, |b, i| b.iter(|| factorial::recursive(*i)));
        group.bench_with_input(BenchmarkId::new("Recursive (with tail)", i), i, |b, i| b.iter(|| factorial::recursive_with_tail(*i)));
    }
    group.finish()
}

criterion_group!(benches, benchmark_variants);
criterion_main!(benches);

