use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../input/2024/day10.txt");
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("trailhead_memchr", |b| {
        b.iter(|| {
            for trailhead in aoc_2024::day10::trailhead_memchr(INPUT.as_bytes()) {
                black_box(trailhead);
            }
        })
    });
    c.bench_function("trailhead_iterator", |b| {
        b.iter(|| {
            for trailhead in aoc_2024::day10::trailhead_iterator(INPUT.as_bytes()) {
                black_box(trailhead);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
