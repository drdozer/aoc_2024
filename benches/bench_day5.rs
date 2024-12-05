use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{self, Rng};

pub fn benchmark_parse_page(c: &mut Criterion) {
    const INPUT: &[u8] = b"01234567898";

    c.bench_function("parse_page_1", |b| {
        let mut rng = rand::thread_rng();
        let input = [
            INPUT[rng.gen_range(0..INPUT.len())],
            INPUT[rng.gen_range(0..INPUT.len())],
        ];
        b.iter(move || parse_page_1(black_box(&input), 0))
    });

    c.bench_function("parse_page_2", |b| {
        let mut rng = rand::thread_rng();
        let input = [
            INPUT[rng.gen_range(0..INPUT.len())],
            INPUT[rng.gen_range(0..INPUT.len())],
        ];
        b.iter(move || parse_page_1(black_box(&input), 0))
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageNumber(u8);

fn parse_page_1(bytes: &[u8], at: usize) -> PageNumber {
    let tens = unsafe { bytes.get_unchecked(at) } - b'0';
    let ones = unsafe { bytes.get_unchecked(at + 1) } - b'0';

    PageNumber(tens * 10 + ones)
}

fn parse_page_2(bytes: &[u8], at: usize) -> PageNumber {
    const correction: u8 = b'0'.wrapping_mul(11);
    let tens = unsafe { bytes.get_unchecked(at) };
    let ones = unsafe { bytes.get_unchecked(at + 1) };

    PageNumber(tens * 10 + ones - correction)
}

criterion_group!(benches, benchmark_parse_page,);
criterion_main!(benches);
