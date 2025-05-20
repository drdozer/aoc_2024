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

impl<P: FullBitset + Copy, const N: usize> FullBitset for PackedBitset<P, N> {
    fn full() -> Self {
        Self([P::full(); N])
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

    fn insert(&mut self, index: usize) -> bool {
        let element_index = self.element_index(index);
        let bit_index = self.bit_index(index);
        self.0[element_index].insert(bit_index)
    }

    fn remove(&mut self, index: usize) {
        let element_index = self.element_index(index);
        let bit_index = self.bit_index(index);
        self.0[element_index].remove(bit_index);
    }

    fn contains(&self, index: usize) -> bool {
        let element_index = self.element_index(index);
        let bit_index = self.bit_index(index);
        self.0[element_index].contains(bit_index)
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for i in 0..N {
            count += self.0[i].count();
        }
        count
    }
}

impl<P: BitsetOps + FixedSizeBitset + BitsetRangeOps + FullBitset, const N: usize> BitsetRangeOps
    for PackedBitset<P, N>
{
    fn insert_range<R: RangeBounds<usize>>(&mut self, range: R) {
        let start = match range.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(i) => *i + 1,
            Bound::Excluded(i) => *i,
            Bound::Unbounded => Self::fixed_capacity(),
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
                    .insert_range(start_bit_index..end_bit_index);
            }
        } else {
            // The update covers multiple elements.

            if start_bit_index > 0 {
                // The edit fell within the first element, so handle the starting fragment.
                unsafe {
                    self.0
                        .get_unchecked_mut(start_element_index)
                        .insert_range(start_bit_index..);
                }
                start_element_index += 1;
            }

            if end_bit_index < Self::fixed_capacity() {
                // The edit fell within the last element, so handle the ending fragment.
                unsafe {
                    self.0
                        .get_unchecked_mut(end_element_index)
                        .insert_range(..end_bit_index);
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

    fn remove_range<R: RangeBounds<usize>>(&mut self, range: R) {
        let start = match range.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(i) => *i + 1,
            Bound::Excluded(i) => *i,
            Bound::Unbounded => Self::fixed_capacity(),
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
                    .remove_range(start_bit_index..end_bit_index);
            }
        } else {
            // The update covers multiple elements.

            if start_bit_index > 0 {
                // The edit fell within the first element, so handle the starting fragment.
                unsafe {
                    self.0
                        .get_unchecked_mut(start_element_index)
                        .remove_range(start_bit_index..);
                }
                start_element_index += 1;
            }

            if end_bit_index < Self::fixed_capacity() {
                // The edit fell within the last element, so handle the ending fragment.
                unsafe {
                    self.0
                        .get_unchecked_mut(end_element_index)
                        .remove_range(..end_bit_index);
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
}

impl<P: FixedSizeBitset + BitsetOpsUnsafe + Copy, const N: usize> BitsetOpsUnsafe
    for PackedBitset<P, N>
{
    unsafe fn insert_unchecked(&mut self, index: usize) -> bool {
        let element_index = self.element_index(index);
        let bit_index = self.bit_index(index);
        self.0
            .get_unchecked_mut(element_index)
            .insert_unchecked(bit_index)
    }

    unsafe fn remove_unchecked(&mut self, index: usize) {
        let element_index = self.element_index(index);
        let bit_index = self.bit_index(index);
        self.0
            .get_unchecked_mut(element_index)
            .remove_unchecked(bit_index);
    }

    unsafe fn contains_unchecked(&self, index: usize) -> bool {
        let element_index = self.element_index(index);
        let bit_index = self.bit_index(index);
        self.0.get_unchecked(element_index).contains_unchecked(bit_index)
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
    type PackedBitsetTest<U> = PackedBitset<PrimitiveBitset<U>, TEST_PACKED_SIZE>;
    
    type PackedBitsetTestU8 = PackedBitsetTest<u8>;
    type PackedBitsetTestU16 = PackedBitsetTest<u16>;
    type PackedBitsetTestU32 = PackedBitsetTest<u32>;
    type PackedBitsetTestU64 = PackedBitsetTest<u64>;
    type PackedBitsetTestU128 = PackedBitsetTest<u128>;

    crate::generate_tests!(test_empty, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);


    crate::generate_tests!(test_set_get, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_set_unset_get, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_unset, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_set_all, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_bitwise_and, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_bitwise_and_assign, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_bitwise_or, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_bitwise_or_assign, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_empty_iterator, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_empty_iterator_back, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_set_one_bit_iterator, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_one_bit_iterator_back, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_set_two_bit_iterator, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);

    crate::generate_tests!(test_set_two_bit_iterator_back, PackedBitsetTestU8, PackedBitsetTestU16, PackedBitsetTestU32, PackedBitsetTestU64, PackedBitsetTestU128);
}
