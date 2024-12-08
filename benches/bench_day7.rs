use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2024::day7::parse_calibration_data;

fn bench_parse_calibration_data(c: &mut Criterion) {
    let input = include_str!("../input/2024/day7.txt");
    c.bench_function("parse_calibration_data", |b| {
        b.iter(|| {
            parse_calibration_data(black_box(input))
                .map(|d| d.test_value)
                .sum::<u64>()
        })
    });
}

criterion_group!(benches, bench_parse_calibration_data,);
criterion_main!(benches);
