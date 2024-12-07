use super::*;
use num::{traits::WrappingSub, One, PrimInt, Unsigned};
use std::fmt::Binary;
use std::iter::IntoIterator;

///- Bitset implementations using a single unsigned integer.
///- This uses generics to support all the unsigned integer types.
///- The implementations assume that you will use all the bits in the underlying integer.
///- They can be composed into bitsets with other behaviours, or used directly.
use std::mem::size_of;
use std::ops::Bound;

/// A bitset implementation that uses a single unsigned integer, and contains one element per bit.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PrimitiveBitset<U> {
    bits: U,
}

impl<U: Binary> std::fmt::Debug for PrimitiveBitset<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PrimitiveBitset({:#b})", self.bits)
    }
}

impl<U> FixedSizeBitset for PrimitiveBitset<U> {
    fn fixed_capacity() -> usize {
        size_of::<U>() * 8
    }
}

impl<U: BitAnd<Output = U>> BitAnd for PrimitiveBitset<U> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
        }
    }
}

impl<U: BitAndAssign> BitAndAssign for PrimitiveBitset<U> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
    }
}

impl<U: BitOr<Output = U>> BitOr for PrimitiveBitset<U> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl<U: BitOrAssign> BitOrAssign for PrimitiveBitset<U> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl<U: BitAnd<Output = U> + BitAndAssign + BitOr<Output = U> + BitOrAssign> BitwiseOps
    for PrimitiveBitset<U>
{
}

impl<U: Unsigned + PrimInt> BitsetOps for PrimitiveBitset<U> {
    fn empty() -> Self {
        Self { bits: U::zero() }
    }

    fn full() -> Self {
        Self {
            bits: U::max_value(),
        }
    }

    fn set(&mut self, index: usize) {
        self.bits = self.bits | U::one() << index;
    }

    fn set_range<R: RangeBounds<usize>>(&mut self, range: R) {
        let start = match range.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(i) => *i + 1,
            Bound::Excluded(i) => *i,
            Bound::Unbounded => self.size(),
        };
        if end >= self.size() {
            self.bits = self.bits | (!U::zero() << start)
        } else {
            self.bits = self.bits | (U::one() << end) - (U::one() << start);
        }
    }

    fn unset(&mut self, index: usize) {
        self.bits = self.bits & !U::one() << index;
    }

    fn unset_range<R: RangeBounds<usize>>(&mut self, range: R) {
        let start = match range.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(i) => *i + 1,
            Bound::Excluded(i) => *i,
            Bound::Unbounded => self.size(),
        };
        if end >= self.size() {
            self.bits = self.bits & !(!U::zero() << start)
        } else {
            let end_mask = U::one() << end;
            let start_mask = U::one() << start;
            self.bits = self.bits & !(end_mask - start_mask);
        }
    }

    fn get(&self, index: usize) -> bool {
        self.bits & U::one() << index != U::zero()
    }

    fn count(&self) -> usize {
        self.bits.count_ones() as usize
    }

    fn size(&self) -> usize {
        Self::fixed_capacity()
    }
}

impl<U: Unsigned + PrimInt> BitsetOpsUnsafe for PrimitiveBitset<U> {
    unsafe fn set_unchecked(&mut self, index: usize) {
        BitsetOps::set(self, index);
    }

    unsafe fn unset_unchecked(&mut self, index: usize) {
        BitsetOps::unset(self, index);
    }

    unsafe fn get_unchecked(&self, index: usize) -> bool {
        BitsetOps::get(self, index)
    }
}

impl<'a, U: Copy + PrimInt + WrappingSub + BitAndAssign + One> IntoIterator
    for &'a PrimitiveBitset<U>
{
    type IntoIter = PrimitiveBitsetIterator<U>;
    type Item = usize;

    fn into_iter(self) -> Self::IntoIter {
        PrimitiveBitsetIterator { bits: self.bits }
    }
}

pub struct PrimitiveBitsetIterator<U> {
    bits: U,
}

impl<U: PrimInt + WrappingSub + BitAndAssign + One> Iterator for PrimitiveBitsetIterator<U> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == U::zero() {
            return None;
        }
        // The trailig zeros always tell us the value of the lowest element of the bitet.
        let value = self.bits.trailing_zeros() as usize;

        // We clear the lowest bit using `bits |& (bits  - 1)`
        // The expression is a mess as we're going through traits.
        self.bits &= self.bits.wrapping_sub(&U::one());

        Some(value)
    }
}

impl<U: PrimInt + WrappingSub + BitAndAssign + One> DoubleEndedIterator
    for PrimitiveBitsetIterator<U>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.bits == U::zero() {
            return None;
        }

        // Get the number of leading zeros
        let leading_zeros = self.bits.leading_zeros() as usize;

        // Calculate the position of the highest set bit
        let value = PrimitiveBitset::<U>::fixed_capacity() - 1 - leading_zeros;

        // Clear the highest bit
        // We can do this by creating a mask with all bits set except the highest set bit
        let mask = (U::one() << value).wrapping_sub(&U::one());
        self.bits &= mask;

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;
    use super::*;

    #[test]
    fn test_empty_u8_bitset() {
        test_empty::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_empty_u16_bitset() {
        test_empty::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_empty_u32_bitset() {
        test_empty::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_empty_u64_bitset() {
        test_empty::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_empty_u128_bitset() {
        test_empty::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_full_u8_bitset() {
        test_full::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_full_u16_bitset() {
        test_full::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_full_u32_bitset() {
        test_full::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_full_u64_bitset() {
        test_full::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_full_u128_bitset() {
        test_full::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_set_get_u8_bitset() {
        test_set_get::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_set_get_u16_bitset() {
        test_set_get::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_set_get_u32_bitset() {
        test_set_get::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_set_get_u64_bitset() {
        test_set_get::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_set_get_u128_bitset() {
        test_set_get::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_set_range_u8_bitset() {
        test_set_range::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_set_range_u16_bitset() {
        test_set_range::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_set_range_u32_bitset() {
        test_set_range::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_set_range_u64_bitset() {
        test_set_range::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_set_range_u128_bitset() {
        test_set_range::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_set_unset_get_u8_bitset() {
        test_set_unset_get::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_set_unset_get_u16_bitset() {
        test_set_unset_get::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_set_unset_get_u32_bitset() {
        test_set_unset_get::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_set_unset_get_u64_bitset() {
        test_set_unset_get::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_set_unset_get_u128_bitset() {
        test_set_unset_get::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_unset_range_u8_bitset() {
        test_unset_range::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_unset_range_u16_bitset() {
        test_unset_range::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_unset_range_u32_bitset() {
        test_unset_range::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_unset_range_u64_bitset() {
        test_unset_range::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_unset_range_u128_bitset() {
        test_unset_range::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_set_all_u8_bitset() {
        test_set_all::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_set_all_u16_bitset() {
        test_set_all::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_set_all_u32_bitset() {
        test_set_all::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_set_all_u64_bitset() {
        test_set_all::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_set_all_u128_bitset() {
        test_set_all::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_bitwise_and_u8_bitset() {
        test_bitwise_and::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_bitwise_and_u16_bitset() {
        test_bitwise_and::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_bitwise_and_u32_bitset() {
        test_bitwise_and::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_bitwise_and_u64_bitset() {
        test_bitwise_and::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_bitwise_and_u128_bitset() {
        test_bitwise_and::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_bitwise_and_assign_u8_bitset() {
        test_bitwise_and_assign::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_bitwise_and_assign_u16_bitset() {
        test_bitwise_and_assign::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_bitwise_and_assign_u32_bitset() {
        test_bitwise_and_assign::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_bitwise_and_assign_u64_bitset() {
        test_bitwise_and_assign::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_bitwise_and_assign_u128_bitset() {
        test_bitwise_and_assign::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_bitwise_or_u8_bitset() {
        test_bitwise_or::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_bitwise_or_u16_bitset() {
        test_bitwise_or::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_bitwise_or_u32_bitset() {
        test_bitwise_or::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_bitwise_or_u64_bitset() {
        test_bitwise_or::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_bitwise_or_u128_bitset() {
        test_bitwise_or::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_bitwise_or_assign_u8_bitset() {
        test_bitwise_or_assign::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_bitwise_or_assign_u16_bitset() {
        test_bitwise_or_assign::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_bitwise_or_assign_u32_bitset() {
        test_bitwise_or_assign::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_bitwise_or_assign_u64_bitset() {
        test_bitwise_or_assign::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_bitwise_or_assign_u128_bitset() {
        test_bitwise_or_assign::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_empty_iterator_u8_bitset() {
        test_empty_iterator::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_empty_iterator_u16_bitset() {
        test_empty_iterator::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_empty_iterator_u32_bitset() {
        test_empty_iterator::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_empty_iterator_u64_bitset() {
        test_empty_iterator::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_empty_iterator_u128_bitset() {
        test_empty_iterator::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_empty_iterator_back_u8_bitset() {
        test_empty_iterator_back::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_empty_iterator_back_u16_bitset() {
        test_empty_iterator_back::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_empty_iterator_back_u32_bitset() {
        test_empty_iterator_back::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_empty_iterator_back_u64_bitset() {
        test_empty_iterator_back::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_empty_iterator_back_u128_bitset() {
        test_empty_iterator_back::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_set_one_bit_iterator_u8_bitset() {
        test_set_one_bit_iterator::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_set_one_bit_iterator_u16_bitset() {
        test_set_one_bit_iterator::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_set_one_bit_iterator_u32_bitset() {
        test_set_one_bit_iterator::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_set_one_bit_iterator_u64_bitset() {
        test_set_one_bit_iterator::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_set_one_bit_iterator_u128_bitset() {
        test_set_one_bit_iterator::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_one_bit_iterator_back_u8_bitset() {
        test_one_bit_iterator_back::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_one_bit_iterator_back_u16_bitset() {
        test_one_bit_iterator_back::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_one_bit_iterator_back_u32_bitset() {
        test_one_bit_iterator_back::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_one_bit_iterator_back_u64_bitset() {
        test_one_bit_iterator_back::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_one_bit_iterator_back_u128_bitset() {
        test_one_bit_iterator_back::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_set_two_bit_iterator_u8_bitset() {
        test_set_two_bit_iterator::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_set_two_bit_iterator_u16_bitset() {
        test_set_two_bit_iterator::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_set_two_bit_iterator_u32_bitset() {
        test_set_two_bit_iterator::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_set_two_bit_iterator_u64_bitset() {
        test_set_two_bit_iterator::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_set_two_bit_iterator_u128_bitset() {
        test_set_two_bit_iterator::<PrimitiveBitset<u128>>();
    }

    #[test]
    fn test_two_bit_iterator_back_u8_bitset() {
        test_set_two_bit_iterator_back::<PrimitiveBitset<u8>>();
    }

    #[test]
    fn test_two_bit_iterator_back_u16_bitset() {
        test_set_two_bit_iterator_back::<PrimitiveBitset<u16>>();
    }

    #[test]
    fn test_two_bit_iterator_back_u32_bitset() {
        test_set_two_bit_iterator_back::<PrimitiveBitset<u32>>();
    }

    #[test]
    fn test_two_bit_iterator_back_u64_bitset() {
        test_set_two_bit_iterator_back::<PrimitiveBitset<u64>>();
    }

    #[test]
    fn test_two_bit_iterator_back_u128_bitset() {
        test_set_two_bit_iterator_back::<PrimitiveBitset<u128>>();
    }
}
