use std::ops::Bound;

///- Bitsets represented as an array of fixed-sized bitsets.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PackedBitset<P, const N: usize>([P; N]);

impl<P: FixedSizeBitset, const N: usize> PackedBitset<P, N> {
    /// Extract the index of the nested bitset corresponding to the index.
    fn element_index(&self, index: usize) -> usize {
        index / P::fixed_capacity()
    }

    // Extract the index of the bit within the nested bitset corresponding to the index.
    fn bit_index(&self, index: usize) -> usize {
        index % P::fixed_capacity()
    }
}

impl<P: FixedSizeBitset, const N: usize> FixedSizeBitset for PackedBitset<P, N> {
    fn fixed_capacity() -> usize {
        N * P::fixed_capacity()
    }
}

impl<P: BitAndAssign + Copy, const N: usize> BitAnd for PackedBitset<P, N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for i in 0..N {
            result.0[i] &= rhs.0[i];
        }
        result
    }
}

impl<P: BitAndAssign + Copy, const N: usize> BitAndAssign for PackedBitset<P, N> {
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.0[i] &= rhs.0[i];
        }
    }
}

impl<P: BitOrAssign + Copy, const N: usize> BitOr for PackedBitset<P, N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for i in 0..N {
            result.0[i] |= rhs.0[i];
        }
        result
    }
}

impl<P: BitOrAssign + Copy, const N: usize> BitOrAssign for PackedBitset<P, N> {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.0[i] |= rhs.0[i];
        }
    }
}

impl<P: BitAndAssign + BitOrAssign + Copy, const N: usize> BitwiseOps for PackedBitset<P, N> {}

impl<P: FixedSizeBitset + BitsetOps + Copy, const N: usize> BitsetOps for PackedBitset<P, N> {
    fn empty() -> Self {
        Self([P::empty(); N])
    }

    fn full() -> Self {
        Self([P::full(); N])
    }

    fn set(&mut self, index: usize) {
        let element_index = self.element_index(index);
        let bit_index = self.bit_index(index);
        self.0[element_index].set(bit_index);
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

        let mut start_element_index = self.element_index(start);
        let mut end_element_index = self.element_index(end);
        let start_bit_index = self.bit_index(start);
        let end_bit_index = self.bit_index(end);

        // If the entire edit is within a single element, we can pass that on.
        if start_element_index == end_element_index {
            unsafe {
                self.0
                    .get_unchecked_mut(start_element_index)
                    .set_range(start_bit_index..end_bit_index);
            }
        } else {
            // The update covers multiple elements.

            if start_bit_index > 0 {
                // The edit fell within the first element, so handle the starting fragment.
                unsafe {
                    self.0
                        .get_unchecked_mut(start_element_index)
                        .set_range(start_bit_index..);
                }
                start_element_index += 1;
            }

            if end_bit_index < Self::fixed_capacity() {
                // The edit fell within the last element, so handle the ending fragment.
                unsafe {
                    self.0
                        .get_unchecked_mut(end_element_index)
                        .set_range(..end_bit_index);
                }
                end_element_index -= 1;
            }

            // Everyting from the start to end is now an entry that needs to be fully set.
            for i in start_element_index..=end_element_index {
                unsafe {
                    *self.0.get_unchecked_mut(i) = P::full();
                }
            }
        }
    }

    fn unset(&mut self, index: usize) {
        let element_index = self.element_index(index);
        let bit_index = self.bit_index(index);
        self.0[element_index].unset(bit_index);
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
        
        let mut start_element_index = self.element_index(start);
        let mut end_element_index = self.element_index(end);
        let start_bit_index = self.bit_index(start);
        let end_bit_index = self.bit_index(end);
        
        // If the entire edit is within a single element, we can pass that on.
        if start_element_index == end_element_index {
            unsafe {
                self.0
                    .get_unchecked_mut(start_element_index)
                    .unset_range(start_bit_index..end_bit_index);
            }
        } else {
            // The update covers multiple elements.
            
            if start_bit_index > 0 {
                // The edit fell within the first element, so handle the starting fragment.
                unsafe {
                    self.0
                        .get_unchecked_mut(start_element_index)
                        .unset_range(start_bit_index..);
                }
                start_element_index += 1;
            }
            
            if end_bit_index < Self::fixed_capacity() {
                // The edit fell within the last element, so handle the ending fragment.
                unsafe {
                    self.0
                        .get_unchecked_mut(end_element_index)
                        .unset_range(..end_bit_index);
                }
                end_element_index -= 1;
            }
            
            // Everyting from the start to end is now an entry that needs to be fully unset.
            for i in start_element_index..=end_element_index {
                unsafe {
                    *self.0.get_unchecked_mut(i) = P::empty();
                }
            }
        }
    }

    fn get(&self, index: usize) -> bool {
        let element_index = self.element_index(index);
        let bit_index = self.bit_index(index);
        self.0[element_index].get(bit_index)
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for i in 0..N {
            count += self.0[i].count();
        }
        count
    }

    fn size(&self) -> usize {
        N * P::fixed_capacity()
    }
}

impl<P: FixedSizeBitset + BitsetOpsUnsafe + Copy, const N: usize> BitsetOpsUnsafe
    for PackedBitset<P, N>
{
    unsafe fn set_unchecked(&mut self, index: usize) {
        let element_index = self.element_index(index);
        let bit_index = self.bit_index(index);
        self.0
            .get_unchecked_mut(element_index)
            .set_unchecked(bit_index);
    }

    unsafe fn unset_unchecked(&mut self, index: usize) {
        let element_index = self.element_index(index);
        let bit_index = self.bit_index(index);
        self.0
            .get_unchecked_mut(element_index)
            .unset_unchecked(bit_index);
    }

    unsafe fn get_unchecked(&self, index: usize) -> bool {
        let element_index = self.element_index(index);
        let bit_index = self.bit_index(index);
        self.0.get_unchecked(element_index).get_unchecked(bit_index)
    }
}

impl<P: FixedSizeBitset + BitsetOps + Copy, const N: usize> Default for PackedBitset<P, N> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<P: FixedSizeBitset, const N: usize> IntoIterator for &PackedBitset<P, N>
where
    for<'a> &'a P: IntoIterator<IntoIter: DoubleEndedIterator<Item = usize>>,
{
    // It is fairly easy to express the iterator logic as an iterator chain.
    // However, this gives rise to a vile type signature.
    // We wrap it behind a newtype to hide the types.
    // Lastly, we use `impl Iterator` in `IntoIter` to prevent the vile type being visible.
    //
    type IntoIter = PackedBitsetIterator<impl DoubleEndedIterator<Item = usize>>;
    type Item = usize;

    fn into_iter(self) -> Self::IntoIter {
        PackedBitsetIterator(self.0.iter().enumerate().flat_map(|(i, p)| {
            let i = i * P::fixed_capacity();
            p.into_iter().map(move |b| i + b)
        }))
    }
}

/// An iterator over the bits of a packed bitset.
pub struct PackedBitsetIterator<I>(I);

impl<I: Iterator<Item = usize>> Iterator for PackedBitsetIterator<I> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<I: DoubleEndedIterator<Item = usize>> DoubleEndedIterator for PackedBitsetIterator<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

#[cfg(test)]
mod tests {
    use super::super::primitives::PrimitiveBitset;
    use super::super::tests::*;
    use super::*;

    const TEST_PACKED_SIZE: usize = 8;

    #[test]
    fn test_empty_packed_u8_bitset() {
        test_empty::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_packed_u16_bitset() {
        test_empty::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_packed_u32_bitset() {
        test_empty::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_packed_u64_bitset() {
        test_empty::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_packed_u128_bitset() {
        test_empty::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_get_packed_u8_bitset() {
        test_set_get::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_get_packed_u16_bitset() {
        test_set_get::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_get_packed_u32_bitset() {
        test_set_get::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_get_packed_u64_bitset() {
        test_set_get::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_get_packed_u128_bitset() {
        test_set_get::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_unset_get_packed_u8_bitset() {
        test_set_unset_get::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_unset_get_packed_u16_bitset() {
        test_set_unset_get::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_unset_get_packed_u32_bitset() {
        test_set_unset_get::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_unset_get_packed_u64_bitset() {
        test_set_unset_get::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_unset_get_packed_u128_bitset() {
        test_set_unset_get::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_all_packed_u8_bitset() {
        test_set_all::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_all_packed_u16_bitset() {
        test_set_all::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_all_packed_u32_bitset() {
        test_set_all::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_all_packed_u64_bitset() {
        test_set_all::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_all_packed_u128_bitset() {
        test_set_all::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_and_packed_u8_bitset() {
        test_bitwise_and::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_and_packed_u16_bitset() {
        test_bitwise_and::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_and_packed_u32_bitset() {
        test_bitwise_and::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_and_packed_u64_bitset() {
        test_bitwise_and::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_and_packed_u128_bitset() {
        test_bitwise_and::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_and_assign_packed_u8_bitset() {
        test_bitwise_and_assign::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_and_assign_packed_u16_bitset() {
        test_bitwise_and_assign::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_and_assign_packed_u32_bitset() {
        test_bitwise_and_assign::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_and_assign_packed_u64_bitset() {
        test_bitwise_and_assign::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_and_assign_packed_u128_bitset() {
        test_bitwise_and_assign::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_or_packed_u8_bitset() {
        test_bitwise_or::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_or_packed_u16_bitset() {
        test_bitwise_or::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_or_packed_u32_bitset() {
        test_bitwise_or::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_or_packed_u64_bitset() {
        test_bitwise_or::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_or_packed_u128_bitset() {
        test_bitwise_or::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_or_assign_packed_u8_bitset() {
        test_bitwise_or_assign::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_or_assign_packed_u16_bitset() {
        test_bitwise_or_assign::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_or_assign_packed_u32_bitset() {
        test_bitwise_or_assign::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_or_assign_packed_u64_bitset() {
        test_bitwise_or_assign::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_bitwise_or_assign_packed_u128_bitset() {
        test_bitwise_or_assign::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_iterator_packed_u8_bitset() {
        test_empty_iterator::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_iterator_packed_u16_bitset() {
        test_empty_iterator::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_iterator_packed_u32_bitset() {
        test_empty_iterator::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_iterator_packed_u64_bitset() {
        test_empty_iterator::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_iterator_packed_u128_bitset() {
        test_empty_iterator::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_iterator_back_packed_u8_bitset() {
        test_empty_iterator_back::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_iterator_back_packed_u16_bitset() {
        test_empty_iterator_back::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_iterator_back_packed_u32_bitset() {
        test_empty_iterator_back::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_iterator_back_packed_u64_bitset() {
        test_empty_iterator_back::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_empty_iterator_back_packed_u128_bitset() {
        test_empty_iterator_back::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_one_bit_iterator_packed_u8_bitset() {
        test_set_one_bit_iterator::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_one_bit_iterator_packed_u16_bitset() {
        test_set_one_bit_iterator::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_one_bit_iterator_packed_u32_bitset() {
        test_set_one_bit_iterator::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_one_bit_iterator_packed_u64_bitset() {
        test_set_one_bit_iterator::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_one_bit_iterator_packed_u128_bitset() {
        test_set_one_bit_iterator::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_one_bit_iterator_back_packed_u8_bitset() {
        test_one_bit_iterator_back::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_one_bit_iterator_back_packed_u16_bitset() {
        test_one_bit_iterator_back::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_one_bit_iterator_back_packed_u32_bitset() {
        test_one_bit_iterator_back::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_one_bit_iterator_back_packed_u64_bitset() {
        test_one_bit_iterator_back::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_one_bit_iterator_back_packed_u128_bitset() {
        test_one_bit_iterator_back::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_two_bit_iterator_packed_u8_bitset() {
        test_set_two_bit_iterator::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_two_bit_iterator_packed_u16_bitset() {
        test_set_two_bit_iterator::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_two_bit_iterator_packed_u32_bitset() {
        test_set_two_bit_iterator::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_two_bit_iterator_packed_u64_bitset() {
        test_set_two_bit_iterator::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_set_two_bit_iterator_packed_u128_bitset() {
        test_set_two_bit_iterator::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_two_bit_iterator_back_packed_u8_bitset() {
        test_set_two_bit_iterator_back::<PackedBitset<PrimitiveBitset<u8>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_two_bit_iterator_back_packed_u16_bitset() {
        test_set_two_bit_iterator_back::<PackedBitset<PrimitiveBitset<u16>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_two_bit_iterator_back_packed_u32_bitset() {
        test_set_two_bit_iterator_back::<PackedBitset<PrimitiveBitset<u32>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_two_bit_iterator_back_packed_u64_bitset() {
        test_set_two_bit_iterator_back::<PackedBitset<PrimitiveBitset<u64>, TEST_PACKED_SIZE>>();
    }

    #[test]
    fn test_two_bit_iterator_back_packed_u128_bitset() {
        test_set_two_bit_iterator_back::<PackedBitset<PrimitiveBitset<u128>, TEST_PACKED_SIZE>>();
    }
}
