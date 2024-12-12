use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2024::day11::{count_digits_loop, count_digits_table, count_digits_ilog10};

fn bench_count_digits(c: &mut Criterion) {
    let inputs = [2, 35, 453, 7645, 65465, 120485];
    c.bench_function("count_digits_loop", |b| {
        b.iter(|| {
            for n in black_box(&inputs) {
                black_box(count_digits_loop(*n));
            }
        })
    });

    c.bench_function("count_digits_table", |b| {
        b.iter(|| {
            for n in black_box(&inputs) {
                black_box(count_digits_table(*n));
            }
        })
    });
    
    c.bench_function("count_digits_ilog10", |b| {
        b.iter(|| {
            for n in black_box(&inputs) {
                black_box(count_digits_ilog10(*n));
            }
        })
    });
}

criterion_group!(benches, bench_count_digits);
criterion_main!(benches);
