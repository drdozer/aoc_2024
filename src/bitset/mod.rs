use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, RangeBounds};

pub mod packed;
pub mod primitives;
pub mod simd;
pub mod sparse;

/// The fundamental bitset operations.
pub trait BitsetOps {
    fn empty() -> Self;
    fn insert(&mut self, index: usize) -> bool;
    fn remove(&mut self, index: usize);
    fn contains(&self, index: usize) -> bool;
    fn count(&self) -> usize;
}

pub trait BitsetRangeOps {
    fn insert_range<R: RangeBounds<usize>>(&mut self, range: R);
    fn remove_range<R: RangeBounds<usize>>(&mut self, range: R);
}

pub trait BitsetOpsUnsafe {
    unsafe fn insert_unchecked(&mut self, index: usize) -> bool;
    unsafe fn remove_unchecked(&mut self, index: usize);
    unsafe fn contains_unchecked(&self, index: usize) -> bool;
}

/// Bitsets that support logical operations.
pub trait BitwiseOps:
    Sized + BitAnd<Output = Self> + BitAndAssign + BitOr<Output = Self> + BitOrAssign
{
}

/// A bitset that can not change the number of bits it contains.
pub trait FixedSizeBitset {
    /// The fixed number of bits in this bitset.
    fn fixed_capacity() -> usize;
}

pub trait FullBitset {
    fn full() -> Self;
}

pub type U8Bitset = primitives::PrimitiveBitset<u8>;
pub type U16Bitset = primitives::PrimitiveBitset<u16>;
pub type U32Bitset = primitives::PrimitiveBitset<u32>;
pub type U64Bitset = primitives::PrimitiveBitset<u64>;
pub type U128Bitset = primitives::PrimitiveBitset<u128>;

pub type PackedU8Bitset<const N: usize> = packed::PackedBitset<U8Bitset, N>;
pub type PackedU16Bitset<const N: usize> = packed::PackedBitset<U16Bitset, N>;
pub type PackedU32Bitset<const N: usize> = packed::PackedBitset<U32Bitset, N>;
pub type PackedU64Bitset<const N: usize> = packed::PackedBitset<U64Bitset, N>;
pub type PackedU128Bitset<const N: usize> = packed::PackedBitset<U128Bitset, N>;

// SIMD bitset with fixed lane counts
pub type SimdU8Bitset2 = simd::SimdU8Bitset2;
pub type SimdU8Bitset4 = simd::SimdU8Bitset4;
pub type SimdU8Bitset8 = simd::SimdU8Bitset8;
pub type SimdU8Bitset16 = simd::SimdU8Bitset16;
pub type SimdU8Bitset32 = simd::SimdU8Bitset32;

pub type SimdU16Bitset2 = simd::SimdU16Bitset2;
pub type SimdU16Bitset4 = simd::SimdU16Bitset4;
pub type SimdU16Bitset8 = simd::SimdU16Bitset8;
pub type SimdU16Bitset16 = simd::SimdU16Bitset16;

pub type SimdU32Bitset2 = simd::SimdU32Bitset2;
pub type SimdU32Bitset4 = simd::SimdU32Bitset4;
pub type SimdU32Bitset8 = simd::SimdU32Bitset8;

pub type SimdU64Bitset2 = simd::SimdU64Bitset2;
pub type SimdU64Bitset4 = simd::SimdU64Bitset4;

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! generate_tests {
        ($test_func:ident, $($type_name:ty),* $(,)?) => {
            paste::paste! {
                $(
                    #[test]
                    #[allow(non_snake_case)]
                    fn [<$test_func _ $type_name>]() {
                        $test_func::<$type_name>();
                    }
                )*
            }
        }
    }

    pub fn test_empty<BS: BitsetOps + FixedSizeBitset>() {
        let empty = BS::empty();
        assert_eq!(empty.count(), 0, "empty bitset should have count 0");

        for i in 0..BS::fixed_capacity() {
            assert!(!empty.contains(i), "empty bitset should not have any bits set");
        }
    }

    pub fn test_full<BS: BitsetOps + FixedSizeBitset + FullBitset>() {
        let full = BS::full();
        assert_eq!(
            full.count(),
            BS::fixed_capacity(),
            "full bitset should have count equal to capacity"
        );
        for i in 0..BS::fixed_capacity() {
            assert!(full.contains(i), "full bitset should have all bits set");
        }
    }

    pub fn test_set_get<BS: BitsetOps + FixedSizeBitset>() {
        for i in 0..BS::fixed_capacity() {
            let mut bitset = BS::empty();
            let was_set = bitset.insert(i);
            assert!(
                was_set,
                "setting an unset bit in bitset should have returned true"
            );
            let was_set = bitset.insert(i);
            assert!(
                !was_set,
                "setting a set bit in bitset should have returned false"
            );
            assert!(bitset.contains(i), "bitset should have bit i set");
            assert_eq!(bitset.count(), 1, "bitset should have count 1");
        }
    }

    pub fn test_unset<BS: BitsetOps + FixedSizeBitset + FullBitset>() {
        let mut bitset = BS::full();
        bitset.remove(5);
        assert!(!bitset.contains(5));
        for i in (0..BS::fixed_capacity()).filter(|&i| i != 5) {
            assert!(bitset.contains(i));
        }
    }

    pub fn test_set_range<BS: BitsetOps + BitsetRangeOps + FixedSizeBitset>() {
        let mut bitset = BS::empty();
        bitset.insert_range(0..BS::fixed_capacity());
        assert_eq!(bitset.count(), BS::fixed_capacity());

        let mut bitset = BS::empty();
        bitset.insert_range(2..5);
        assert_eq!(bitset.count(), 3);
    }

    pub fn test_set_unset_get<BS: BitsetOps + FixedSizeBitset>() {
        for i in 0..BS::fixed_capacity() {
            let mut bitset = BS::empty();
            let was_set = bitset.insert(i);
            assert!(
                was_set,
                "setting an unset bit in bitset should have returned true"
            );
            let was_set = bitset.insert(i);
            assert!(
                !was_set,
                "setting a set bit in bitset should have returned false"
            );
            bitset.remove(i);
            assert!(!bitset.contains(i), "bitset should not have bit {} unset", i);
            assert_eq!(bitset.count(), 0, "bitset should have count {}", i);
        }
    }

    pub fn test_unset_range<BS: BitsetOps + BitsetRangeOps + FixedSizeBitset + std::fmt::Debug>() {
        // The full bitset range set and unset
        let mut bitset = BS::empty();
        bitset.insert_range(0..BS::fixed_capacity());
        bitset.remove_range(0..BS::fixed_capacity());
        assert_eq!(bitset.count(), 0);

        // Set some and unset some
        let mut bitset = BS::empty();
        bitset.insert_range(1..BS::fixed_capacity() - 1);
        assert_eq!(
            bitset.count(),
            BS::fixed_capacity() - 2,
            "bitset should have count {} in {:?}",
            BS::fixed_capacity() - 1,
            bitset
        );
        bitset.remove_range(2..BS::fixed_capacity() - 2);
        assert_eq!(
            bitset.count(),
            2,
            "bitset should have count 2 in {:?}",
            bitset
        );
    }

    pub fn test_set_all<BS: BitsetOps + FixedSizeBitset>() {
        let mut bitset = BS::empty();
        for i in 0..BS::fixed_capacity() {
            bitset.insert(i);
            assert_eq!(bitset.count() as usize, i + 1);
        }
    }

    pub fn test_bitwise_and<BS: BitsetOps + BitwiseOps + FixedSizeBitset + Eq + std::fmt::Debug>() {
        for i in 0..BS::fixed_capacity() {
            let mut bitset1 = BS::empty();
            let mut bitset2 = BS::empty();
            let mut bitset3 = BS::empty();
            bitset1.insert(i);
            bitset2.insert(i);
            bitset3.insert(i);
            assert_eq!(bitset1 & bitset2, bitset3);
        }
    }

    pub fn test_bitwise_and_assign<
        BS: BitsetOps + BitwiseOps + FixedSizeBitset + Eq + std::fmt::Debug,
    >() {
        for i in 0..BS::fixed_capacity() {
            let mut bitset1 = BS::empty();
            let mut bitset2 = BS::empty();
            let mut bitset3 = BS::empty();
            bitset1.insert(i);
            bitset2.insert(i);
            bitset3.insert(i);

            bitset1 &= bitset2;
            assert_eq!(bitset1, bitset3);
        }
    }

    pub fn test_bitwise_or<BS: BitsetOps + BitwiseOps + FixedSizeBitset + Eq + std::fmt::Debug>() {
        for i in 0..BS::fixed_capacity() {
            for j in 0..BS::fixed_capacity() {
                let mut bitset1 = BS::empty();
                let mut bitset2 = BS::empty();
                let mut bitset3 = BS::empty();
                bitset1.insert(i);
                bitset2.insert(j);
                bitset3.insert(i);
                bitset3.insert(j);
                assert_eq!(bitset1 | bitset2, bitset3);
            }
        }
    }
    pub fn test_bitwise_or_assign<
        BS: BitsetOps + BitwiseOps + FixedSizeBitset + Eq + std::fmt::Debug,
    >() {
        for i in 0..BS::fixed_capacity() {
            for j in 0..BS::fixed_capacity() {
                let mut bitset1 = BS::empty();
                let mut bitset2 = BS::empty();
                let mut bitset3 = BS::empty();
                bitset1.insert(i);
                bitset2.insert(j);
                bitset3.insert(i);
                bitset3.insert(j);

                bitset1 |= bitset2;
                assert_eq!(bitset1, bitset3);
            }
        }
    }

    pub fn test_empty_iterator<BS: BitsetOps>()
    where
        for<'a> &'a BS: IntoIterator<Item = usize>,
    {
        let empty = BS::empty();
        let mut iter = empty.into_iter();
        assert_eq!(iter.next(), None);
    }

    pub fn test_empty_iterator_back<BS: BitsetOps>()
    where
        for<'a> &'a BS: IntoIterator<IntoIter: DoubleEndedIterator<Item = usize>>,
    {
        let empty = BS::empty();
        let mut iter = empty.into_iter();
        assert_eq!(iter.next_back(), None);
    }

    pub fn test_set_one_bit_iterator<BS: BitsetOps + FixedSizeBitset>()
    where
        for<'a> &'a BS: IntoIterator<Item = usize>,
    {
        for i in 0..BS::fixed_capacity() {
            let mut bitset = BS::empty();
            bitset.insert(i);
            let mut iter = bitset.into_iter();
            assert_eq!(iter.next(), Some(i));
            assert_eq!(iter.next(), None);
        }
    }

    pub fn test_one_bit_iterator_back<BS: BitsetOps + FixedSizeBitset>()
    where
        for<'a> &'a BS: IntoIterator<IntoIter: DoubleEndedIterator<Item = usize>>,
    {
        for i in 0..BS::fixed_capacity() {
            let mut bitset = BS::empty();
            bitset.insert(i);
            let mut iter = bitset.into_iter();
            assert_eq!(iter.next_back(), Some(i));
            assert_eq!(iter.next_back(), None);
        }
    }

    pub fn test_set_two_bit_iterator<BS: BitsetOps + FixedSizeBitset>()
    where
        for<'a> &'a BS: IntoIterator<Item = usize>,
    {
        for i in 0..BS::fixed_capacity() {
            for j in i + 1..BS::fixed_capacity() {
                let mut bitset = BS::empty();
                bitset.insert(i);
                bitset.insert(j);
                let mut iter = bitset.into_iter();
                assert_eq!(iter.next(), Some(i));
                assert_eq!(iter.next(), Some(j));
                assert_eq!(iter.next(), None);
            }
        }
    }

    pub fn test_set_two_bit_iterator_back<BS: BitsetOps + FixedSizeBitset>()
    where
        for<'a> &'a BS: IntoIterator<IntoIter: DoubleEndedIterator<Item = usize>>,
    {
        for i in 0..BS::fixed_capacity() {
            for j in i + 1..BS::fixed_capacity() {
                let mut bitset = BS::empty();
                bitset.insert(i);
                bitset.insert(j);
                let mut iter = bitset.into_iter();
                assert_eq!(iter.next_back(), Some(j));
                assert_eq!(iter.next_back(), Some(i));
                assert_eq!(iter.next_back(), None);
            }
        }
    }
}
