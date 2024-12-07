use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2024::day6::parse_lab_map;

fn bench_parse_lab_map(c: &mut Criterion) {
    let input = include_str!("../input/2024/day6.txt");
    c.bench_function("parse_lab_map", |b| {
        b.iter(|| parse_lab_map(black_box(input)))
    });
}

criterion_group!(benches, bench_parse_lab_map,);
criterion_main!(benches);
