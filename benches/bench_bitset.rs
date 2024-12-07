use aoc_2024::bitset::{BitsetOps, BitsetOpsUnsafe, FixedSizeBitset};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{self, Rng};

fn benchmark_set_get_unset_bit<BS: BitsetOps + FixedSizeBitset>(c: &mut Criterion, name: &str) {
    let mut rng = rand::thread_rng();

    c.bench_function(format!("set_get_unset_bit {}", name).as_str(), |b| {
        let i = rng.gen_range(0..BS::fixed_capacity());
        let mut bs = BS::empty();
        b.iter(move || {
            bs.set(i);
            let a = bs.get(i);
            bs.unset(i);
            a
        })
    });
}

fn benchmark_unsafe_set_get_unset_bit<BS: BitsetOps + BitsetOpsUnsafe + FixedSizeBitset>(
    c: &mut Criterion,
    name: &str,
) {
    let mut rng = rand::thread_rng();

    c.bench_function(format!("unsafe_set_get_unset_bit {}", name).as_str(), |b| {
        let i = rng.gen_range(0..BS::fixed_capacity());
        let mut bs = BS::empty();
        b.iter(move || unsafe {
            bs.set_unchecked(i);
            let a = bs.get_unchecked(i);
            bs.unset_unchecked(i);
            a
        })
    });
}

fn benchmark_matrix_access<BS: BitsetOps + FixedSizeBitset + Copy>(c: &mut Criterion, name: &str) {
    let mut rng = rand::thread_rng();

    c.bench_function(format!("matrix_access {}", name).as_str(), |b| {
        let i = rng.gen_range(0..BS::fixed_capacity());
        let mut bs = [BS::empty(); 130];
        b.iter(move || {
            let mut a = false;
            for j in 0..130 {
                bs[j].set(i);
                a = bs[j].get(i);
                bs[j].unset(i);
            }
            a
        })
    });
}

fn benchmark_unsafe_matrix_access<BS: BitsetOps + BitsetOpsUnsafe + FixedSizeBitset + Copy>(
    c: &mut Criterion,
    name: &str,
) {
    let mut rng = rand::thread_rng();

    c.bench_function(format!("unsafe_matrix_access {}", name).as_str(), |b| {
        let i = rng.gen_range(0..BS::fixed_capacity());
        let mut bs = [BS::empty(); 130];
        b.iter(move || unsafe {
            let mut a = false;
            for j in 0..130 {
                bs.get_unchecked_mut(j).set_unchecked(i);
                a = bs.get_unchecked(j).get_unchecked(i);
                bs.get_unchecked_mut(j).unset_unchecked(i);
            }
            a
        })
    });
}

fn benchmark_set_get_unset_bit_primitives(c: &mut Criterion) {
    benchmark_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u8>>(
        c,
        "PrimitiveBitSet<u8>",
    );
    benchmark_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u16>>(
        c,
        "PrimitiveBitSet<u16>",
    );
    benchmark_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u32>>(
        c,
        "PrimitiveBitSet<u32>",
    );
    benchmark_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u64>>(
        c,
        "PrimitiveBitSet<u64>",
    );
    benchmark_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u128>>(
        c,
        "PrimitiveBitSet<u128>",
    );
}

fn benchmark_unsafe_set_get_unset_bit_primitives(c: &mut Criterion) {
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u8>>(
        c,
        "PrimitiveBitSet<u8>",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u16>>(
        c,
        "PrimitiveBitSet<u16>",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u32>>(
        c,
        "PrimitiveBitSet<u32>",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u64>>(
        c,
        "PrimitiveBitSet<u64>",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u128>>(
        c,
        "PrimitiveBitSet<u128>",
    );
}

fn benchmark_set_get_unset_bit_packed_8(c: &mut Criterion) {
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU8Bitset<8>>(c, "PackedBitset u8, 8");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u16>>(
        c,
        "PackedBitset u16, 8",
    );
    benchmark_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u32>>(
        c,
        "PackedBitset u32, 8",
    );
    benchmark_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u64>>(
        c,
        "PackedBitset u64, 8",
    );
    benchmark_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u128>>(
        c,
        "PackedBitset u128, 8",
    );
}

fn benchmark_unsafe_set_get_unset_bit_packed_8(c: &mut Criterion) {
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU8Bitset<8>>(
        c,
        "PackedBitset u8, 8",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u16>>(
        c,
        "PackedBitset u16, 8",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u32>>(
        c,
        "PackedBitset u32, 8",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u64>>(
        c,
        "PackedBitset u64, 8",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::primitives::PrimitiveBitset<u128>>(
        c,
        "PackedBitset u128, 8",
    );
}

fn benchmark_set_get_unset_bit_packed_64(c: &mut Criterion) {
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU8Bitset<64>>(c, "PackedBitset u8, 64");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU16Bitset<64>>(c, "PackedBitset u16, 64");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU32Bitset<64>>(c, "PackedBitset u32, 64");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU64Bitset<64>>(c, "PackedBitset u64, 64");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU128Bitset<64>>(
        c,
        "PackedBitset u128, 64",
    );
}

fn benchmark_unsafe_set_get_unset_bit_packed_64(c: &mut Criterion) {
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU8Bitset<64>>(
        c,
        "PackedBitset u8, 64",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU16Bitset<64>>(
        c,
        "PackedBitset u16, 64",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU32Bitset<64>>(
        c,
        "PackedBitset u32, 64",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU64Bitset<64>>(
        c,
        "PackedBitset u64, 64",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU128Bitset<64>>(
        c,
        "PackedBitset u128, 64",
    );
}

fn benchmark_set_get_unset_bit_packed_fixed_size(c: &mut Criterion) {
    // The fixed size will be 4 x u128
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU128Bitset<4>>(c, "PackedBitset u128, 4");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU64Bitset<8>>(c, "PackedBitset u64, 8");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU32Bitset<16>>(c, "PackedBitset u32, 16");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU16Bitset<32>>(c, "PackedBitset u16, 32");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU8Bitset<64>>(c, "PackedBitset u8, 64");
}

fn benchmark_unsafe_set_get_unset_bit_packed_fixed_size(c: &mut Criterion) {
    // The fixed size will be 4 x u128
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU128Bitset<4>>(
        c,
        "PackedBitset u128, 4",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU64Bitset<8>>(
        c,
        "PackedBitset u64, 8",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU32Bitset<16>>(
        c,
        "PackedBitset u32, 16",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU16Bitset<32>>(
        c,
        "PackedBitset u16, 32",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU8Bitset<64>>(
        c,
        "PackedBitset u8, 64",
    );
}

fn benchmark_set_get_unset_bit_packed_17_bytes(c: &mut Criterion) {
    // The size that must be covered is 17 bytes
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU128Bitset<2>>(c, "PackedBitset u128, 2");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU64Bitset<3>>(c, "PackedBitset u64, 3");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU32Bitset<5>>(c, "PackedBitset u32, 5");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU16Bitset<9>>(c, "PackedBitset u16, 9");
    benchmark_set_get_unset_bit::<aoc_2024::bitset::PackedU8Bitset<17>>(c, "PackedBitset u8, 17");
}

fn benchmark_unsafe_set_get_unset_bit_packed_17_bytes(c: &mut Criterion) {
    // The size that must be covered is 17 bytes
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU128Bitset<2>>(
        c,
        "PackedBitset u128, 2",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU64Bitset<3>>(
        c,
        "PackedBitset u64, 3",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU32Bitset<5>>(
        c,
        "PackedBitset u32, 5",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU16Bitset<9>>(
        c,
        "PackedBitset u16, 9",
    );
    benchmark_unsafe_set_get_unset_bit::<aoc_2024::bitset::PackedU8Bitset<17>>(
        c,
        "PackedBitset u8, 17",
    );
}

fn benchmark_matrix_access_packed_17_bytes(c: &mut Criterion) {
    benchmark_matrix_access::<aoc_2024::bitset::PackedU128Bitset<2>>(c, "PackedBitset u128, 2");
    benchmark_matrix_access::<aoc_2024::bitset::PackedU64Bitset<3>>(c, "PackedBitset u64, 3");
    benchmark_matrix_access::<aoc_2024::bitset::PackedU32Bitset<5>>(c, "PackedBitset u32, 5");
    benchmark_matrix_access::<aoc_2024::bitset::PackedU16Bitset<9>>(c, "PackedBitset u16, 9");
    benchmark_matrix_access::<aoc_2024::bitset::PackedU8Bitset<17>>(c, "PackedBitset u8, 17");
}

fn benchmark_unsafe_matrix_access_packed_17_bytes(c: &mut Criterion) {
    benchmark_unsafe_matrix_access::<aoc_2024::bitset::PackedU128Bitset<2>>(
        c,
        "PackedBitset u128, 2",
    );
    benchmark_unsafe_matrix_access::<aoc_2024::bitset::PackedU64Bitset<3>>(
        c,
        "PackedBitset u64, 3",
    );
    benchmark_unsafe_matrix_access::<aoc_2024::bitset::PackedU32Bitset<5>>(
        c,
        "PackedBitset u32, 5",
    );
    benchmark_unsafe_matrix_access::<aoc_2024::bitset::PackedU16Bitset<9>>(
        c,
        "PackedBitset u16, 9",
    );
    benchmark_unsafe_matrix_access::<aoc_2024::bitset::PackedU8Bitset<17>>(
        c,
        "PackedBitset u8, 17",
    );
}

criterion_group!(
    benches,
    benchmark_set_get_unset_bit_primitives,
    benchmark_set_get_unset_bit_packed_8,
    benchmark_set_get_unset_bit_packed_64,
    benchmark_set_get_unset_bit_packed_fixed_size,
    benchmark_unsafe_set_get_unset_bit_primitives,
    benchmark_unsafe_set_get_unset_bit_packed_8,
    benchmark_unsafe_set_get_unset_bit_packed_64,
    benchmark_unsafe_set_get_unset_bit_packed_fixed_size,
    benchmark_set_get_unset_bit_packed_17_bytes,
    benchmark_unsafe_set_get_unset_bit_packed_17_bytes,
    benchmark_matrix_access_packed_17_bytes,
    benchmark_unsafe_matrix_access_packed_17_bytes,
);

criterion_main!(benches);
