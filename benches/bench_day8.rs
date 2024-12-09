use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2024::day8::{
    antenna_to_index_usize_early, antenna_to_index_usize_late, antenna_to_index_usize_mid, parse_rc, parse_skip, part1_solve_enumerated, part1_solve_rc, MAP_SIZE
};

fn bench_parse_input(c: &mut Criterion) {
    let input = include_str!("../input/2024/day8.txt");
    c.bench_function("skip parser", |b| {
        b.iter(|| {
            for s in parse_skip(black_box(input)) {
                black_box(s);
            }
        })
    });
    c.bench_function("rc parser", |b| {
        b.iter(|| {
            for b in parse_rc(black_box(input)) {
                black_box(b);
            }
        })
    });
    c.bench_function("null parser", |b| {
        b.iter(|| {
            for b in black_box(input).as_bytes() {
                black_box(b);
            }
        })
    });
    c.bench_function("enumerate parser", |b| {
        b.iter(|| {
            for b in black_box(input).as_bytes().iter().enumerate() {
                black_box(b);
            }
        })
    });

    c.bench_function("rc parser collect", |b| {
        b.iter(|| {
            parse_rc(black_box(input)).collect::<Vec<_>>();
        })
    });
}

fn bench_antenna_to_index(c: &mut Criterion) {
    c.bench_function("antenna_to_index_usize_early", |b| {
        b.iter(|| {
            for antenna in b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" {
                black_box(antenna_to_index_usize_early(black_box(*antenna)));
            }
        })
    });

    c.bench_function("antenna_to_index_usize_mid", |b| {
        b.iter(|| {
            for antenna in b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" {
                black_box(antenna_to_index_usize_mid(black_box(*antenna)));
            }
        })
    });

    c.bench_function("antenna_to_index_usize_late", |b| {
        b.iter(|| {
            for antenna in b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" {
                black_box(antenna_to_index_usize_late(black_box(*antenna)));
            }
        })
    });
}

fn bench_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day8.txt");
    c.bench_function("part 1 using rc", |b| {
        b.iter(|| {
            part1_solve_rc(input, MAP_SIZE)
        })
    });
    c.bench_function("part 1 using enumerated", |b| {
        b.iter(|| {
            part1_solve_enumerated(input, MAP_SIZE)
        })
    });
    c.bench_function("part 1 using enumerated2", |b| {
        b.iter(|| {
            part1_solve_enumerated(input, MAP_SIZE)
        })
    });
}

criterion_group!(benches, bench_parse_input, bench_antenna_to_index, bench_part1);
criterion_main!(benches);
