use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, RangeBounds};

pub mod packed;
pub mod primitives;

/// The fundamental bitset operations.
pub trait BitsetOps {
    fn empty() -> Self;
    fn full() -> Self;
    fn set(&mut self, index: usize) -> bool;
    fn set_range<R: RangeBounds<usize>>(&mut self, range: R);
    fn unset(&mut self, index: usize);
    fn unset_range<R: RangeBounds<usize>>(&mut self, range: R);
    fn get(&self, index: usize) -> bool;
    fn count(&self) -> usize;
    fn size(&self) -> usize;
}

pub trait BitsetOpsUnsafe {
    unsafe fn set_unchecked(&mut self, index: usize) -> bool;
    unsafe fn unset_unchecked(&mut self, index: usize);
    unsafe fn get_unchecked(&self, index: usize) -> bool;
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

    pub fn test_empty<BS: BitsetOps>() {
        let empty = BS::empty();
        assert_eq!(empty.count(), 0, "empty bitset should have count 0");

        for i in 0..empty.size() {
            assert!(!empty.get(i), "empty bitset should not have any bits set");
        }
    }

    pub fn test_full<BS: BitsetOps>() {
        let full = BS::full();
        assert_eq!(
            full.count(),
            full.size(),
            "full bitset should have count equal to size"
        );
        for i in 0..full.size() {
            assert!(full.get(i), "full bitset should have all bits set");
        }
    }

    pub fn test_set_get<BS: BitsetOps>() {
        for i in 0..BS::empty().size() {
            let mut bitset = BS::empty();
            let was_set = bitset.set(i);
            assert!(
                was_set,
                "setting an unset bit in bitset should have returned true"
            );
            let was_set = bitset.set(i);
            assert!(
                !was_set,
                "setting a set bit in bitset should have returned false"
            );
            assert!(bitset.get(i), "bitset should have bit i set");
            assert_eq!(bitset.count(), 1, "bitset should have count 1");
        }
    }

    pub fn test_set_range<BS: BitsetOps>() {
        let mut bitset = BS::empty();
        bitset.set_range(0..bitset.size());
        assert_eq!(bitset.count(), bitset.size());

        let mut bitset = BS::empty();
        bitset.set_range(2..5);
        assert_eq!(bitset.count(), 3);
    }

    pub fn test_set_unset_get<BS: BitsetOps>() {
        for i in 0..BS::empty().size() {
            let mut bitset = BS::empty();
            let was_set = bitset.set(i);
            assert!(
                was_set,
                "setting an unset bit in bitset should have returned true"
            );
            let was_set = bitset.set(i);
            assert!(
                !was_set,
                "setting a set bit in bitset should have returned false"
            );
            bitset.unset(i);
            assert!(!bitset.get(i), "bitset should not have bit {} unset", i);
            assert_eq!(bitset.count(), 0, "bitset should have count {}", i);
        }
    }

    pub fn test_unset_range<BS: BitsetOps + std::fmt::Debug>() {
        // The full bitset range set and unset
        let mut bitset = BS::empty();
        bitset.set_range(0..bitset.size());
        bitset.unset_range(0..bitset.size());
        assert_eq!(bitset.count(), 0);

        // Set some and unset some
        let mut bitset = BS::empty();
        bitset.set_range(1..bitset.size() - 1);
        assert_eq!(
            bitset.count(),
            bitset.size() - 2,
            "bitset should have count {} in {:?}",
            bitset.size() - 1,
            bitset
        );
        bitset.unset_range(2..bitset.size() - 2);
        assert_eq!(
            bitset.count(),
            2,
            "bitset should have count 2 in {:?}",
            bitset
        );
    }

    pub fn test_set_all<BS: BitsetOps>() {
        let mut bitset = BS::empty();
        for i in 0..bitset.size() {
            bitset.set(i);
            assert_eq!(bitset.count() as usize, i + 1);
        }
    }

    pub fn test_bitwise_and<BS: BitsetOps + BitwiseOps + Eq + std::fmt::Debug>() {
        for i in 0..BS::empty().size() {
            let mut bitset1 = BS::empty();
            let mut bitset2 = BS::empty();
            let mut bitset3 = BS::empty();
            bitset1.set(i);
            bitset2.set(i);
            bitset3.set(i);
            assert_eq!(bitset1 & bitset2, bitset3);
        }
    }

    pub fn test_bitwise_and_assign<BS: BitsetOps + BitwiseOps + Eq + std::fmt::Debug>() {
        for i in 0..BS::empty().size() {
            let mut bitset1 = BS::empty();
            let mut bitset2 = BS::empty();
            let mut bitset3 = BS::empty();
            bitset1.set(i);
            bitset2.set(i);
            bitset3.set(i);

            bitset1 &= bitset2;
            assert_eq!(bitset1, bitset3);
        }
    }

    pub fn test_bitwise_or<BS: BitsetOps + BitwiseOps + Eq + std::fmt::Debug>() {
        for i in 0..BS::empty().size() {
            for j in 0..BS::empty().size() {
                let mut bitset1 = BS::empty();
                let mut bitset2 = BS::empty();
                let mut bitset3 = BS::empty();
                bitset1.set(i);
                bitset2.set(j);
                bitset3.set(i);
                bitset3.set(j);
                assert_eq!(bitset1 | bitset2, bitset3);
            }
        }
    }
    pub fn test_bitwise_or_assign<BS: BitsetOps + BitwiseOps + Eq + std::fmt::Debug>() {
        for i in 0..BS::empty().size() {
            for j in 0..BS::empty().size() {
                let mut bitset1 = BS::empty();
                let mut bitset2 = BS::empty();
                let mut bitset3 = BS::empty();
                bitset1.set(i);
                bitset2.set(j);
                bitset3.set(i);
                bitset3.set(j);

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

    pub fn test_set_one_bit_iterator<BS: BitsetOps>()
    where
        for<'a> &'a BS: IntoIterator<Item = usize>,
    {
        for i in 0..BS::empty().size() {
            let mut bitset = BS::empty();
            bitset.set(i);
            let mut iter = bitset.into_iter();
            assert_eq!(iter.next(), Some(i));
            assert_eq!(iter.next(), None);
        }
    }

    pub fn test_one_bit_iterator_back<BS: BitsetOps>()
    where
        for<'a> &'a BS: IntoIterator<IntoIter: DoubleEndedIterator<Item = usize>>,
    {
        for i in 0..BS::empty().size() {
            let mut bitset = BS::empty();
            bitset.set(i);
            let mut iter = bitset.into_iter();
            assert_eq!(iter.next_back(), Some(i));
            assert_eq!(iter.next_back(), None);
        }
    }

    pub fn test_set_two_bit_iterator<BS: BitsetOps>()
    where
        for<'a> &'a BS: IntoIterator<Item = usize>,
    {
        for i in 0..BS::empty().size() {
            for j in i + 1..BS::empty().size() {
                let mut bitset = BS::empty();
                bitset.set(i);
                bitset.set(j);
                let mut iter = bitset.into_iter();
                assert_eq!(iter.next(), Some(i));
                assert_eq!(iter.next(), Some(j));
                assert_eq!(iter.next(), None);
            }
        }
    }

    pub fn test_set_two_bit_iterator_back<BS: BitsetOps>()
    where
        for<'a> &'a BS: IntoIterator<IntoIter: DoubleEndedIterator<Item = usize>>,
    {
        for i in 0..BS::empty().size() {
            for j in i + 1..BS::empty().size() {
                let mut bitset = BS::empty();
                bitset.set(i);
                bitset.set(j);
                let mut iter = bitset.into_iter();
                assert_eq!(iter.next_back(), Some(j));
                assert_eq!(iter.next_back(), Some(i));
                assert_eq!(iter.next_back(), None);
            }
        }
    }
}
