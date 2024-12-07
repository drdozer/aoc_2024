use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

pub mod packed;
pub mod primitives;

/// The fundamental bitset operations.
pub trait BitsetOps {
    fn empty() -> Self;
    fn set(&mut self, index: usize);
    fn unset(&mut self, index: usize);
    fn get(&self, index: usize) -> bool;
    fn count(&self) -> u32;
    fn size(&self) -> usize;
}

pub trait BitsetOpsUnsafe {
    unsafe fn set_unchecked(&mut self, index: usize);
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

    pub fn test_empty<BS: BitsetOps>() {
        let empty = BS::empty();
        assert_eq!(empty.count(), 0, "empty bitset should have count 0");

        for i in 0..empty.size() {
            assert!(!empty.get(i), "empty bitset should not have any bits set");
        }
    }

    pub fn test_set_get<BS: BitsetOps>() {
        for i in 0..BS::empty().size() {
            let mut bitset = BS::empty();
            bitset.set(i);
            assert!(bitset.get(i), "bitset should have bit i set");
            assert_eq!(bitset.count(), 1, "bitset should have count 1");
        }
    }

    pub fn test_set_unset_get<BS: BitsetOps>() {
        for i in 0..BS::empty().size() {
            let mut bitset = BS::empty();
            bitset.set(i);
            bitset.unset(i);
            assert!(!bitset.get(i), "bitset should not have bit {} unset", i);
            assert_eq!(bitset.count(), 0, "bitset should have count {}", i);
        }
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
}
