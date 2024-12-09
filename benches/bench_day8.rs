use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2024::day8::{
    antenna_to_index_usize_early, antenna_to_index_usize_late, antenna_to_index_usize_mid,
    parse_rc, parse_skip,
};

fn bench_parse_input(c: &mut Criterion) {
    let input = include_str!("../input/2024/day7.txt");
    c.bench_function("skip parser", |b| {
        b.iter(|| {
            for _ in parse_skip(black_box(input)) {
                black_box(())
            }
        })
    });
    c.bench_function("rc parser", |b| {
        b.iter(|| {
            for _ in parse_rc(black_box(input)) {
                black_box(())
            }
        })
    });
    c.bench_function("null parser", |b| {
        b.iter(|| {
            for _ in black_box(input).as_bytes() {
                black_box(())
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

criterion_group!(benches, bench_parse_input, bench_antenna_to_index);
criterion_main!(benches);
