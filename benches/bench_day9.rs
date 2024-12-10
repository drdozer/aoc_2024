use aoc_2024::day9::{sum_checksum_range, sum_checksum_range_loop};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rand::Rng;

fn benchmark_checksum_comparisons(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // Create a vector of test cases
    let test_cases: Vec<(u64, u64, u64)> = (0..1000)
        .map(|_| {
            (
                rng.gen_range(0..2000), // start
                rng.gen_range(0..10),   // len
                rng.gen_range(0..1000), // id
            )
        })
        .collect();

    c.bench_function("sum_checksum_range", |b| {
        b.iter(|| {
            for &(start, len, id) in test_cases.iter() {
                black_box(sum_checksum_range(
                    black_box(start),
                    black_box(len),
                    black_box(id),
                ));
            }
        })
    });

    c.bench_function("sum_checksum_range_loop", |b| {
        b.iter(|| {
            for &(start, len, id) in test_cases.iter() {
                black_box(sum_checksum_range_loop(
                    black_box(start),
                    black_box(len),
                    black_box(id),
                ));
            }
        })
    });
}

criterion_group!(benches, benchmark_checksum_comparisons);
criterion_main!(benches);
