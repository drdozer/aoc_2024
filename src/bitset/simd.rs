use super::*;
use std::fmt::{Debug, Binary};
use std::iter::IntoIterator;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Bound, Not, Shl};
use std::simd::{Simd, SimdElement, LaneCount, SupportedLaneCount};
use num::{traits::WrappingSub, Zero, One, PrimInt};

/// A bitset implementation using SIMD vector types.
/// This provides efficient bitwise operations on large sets of bits.
#[derive(Clone, PartialEq, Eq, Copy)]
pub struct SimdBitset<T, const N: usize>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount
{
    pub bits: Simd<T, N>,
}

// impl<T, const N: usize> Default for SimdBitset<T, N>
// where
//     T: SimdElement,
//     LaneCount<N>: SupportedLaneCount
// {
//     fn default() -> Self {
//         Self::empty()
//     }
// }

impl<T, const N: usize> Debug for SimdBitset<T, N>
where
    T: SimdElement + Binary,
    LaneCount<N>: SupportedLaneCount,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimdBitset(")?;
        let mut first = true;
        for i in 0..N {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{:b}", self.bits[i])?;
        }
        write!(f, ")")
    }
}

impl<T, const N: usize> FixedSizeBitset for SimdBitset<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    fn fixed_capacity() -> usize {
        N * std::mem::size_of::<T>() * 8
    }
}

impl<T, const N: usize> FullBitset for SimdBitset<T, N>
where
    T: SimdElement + Zero + Not<Output = T>,
    LaneCount<N>: SupportedLaneCount,
{
    fn full() -> Self {
        // Create a value with all bits set
        let all_ones = !T::zero();
        Self {
            bits: Simd::splat(all_ones),
        }
    }
}

impl<T, const N: usize> BitAnd for SimdBitset<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: BitAnd<Output = Simd<T, N>>,
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self { bits: self.bits & rhs.bits }
    }
}

impl<T, const N: usize> BitAndAssign for SimdBitset<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: BitAndAssign<Simd<T, N>>,
{
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
    }
}

impl<T, const N: usize> BitOr for SimdBitset<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: BitOr<Output = Simd<T, N>>,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { bits: self.bits | rhs.bits }
    }
}

impl<T, const N: usize> BitOrAssign for SimdBitset<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: BitOrAssign<Simd<T, N>>,
{
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits
    }
}

impl<T, const N: usize> BitwiseOps for SimdBitset<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: BitAnd<Output = Simd<T, N>> + BitOr<Output = Simd<T, N>>,
{
}

impl<T, const N: usize> BitsetOps for SimdBitset<T, N>
where
    T: SimdElement + Default + PrimInt + BitAndAssign + BitOrAssign,
    LaneCount<N>: SupportedLaneCount,
{
    fn empty() -> Self {
        Self {
            bits: Simd::splat(T::default()),
        }
    }

    fn insert(&mut self, index: usize) -> bool {
        let element_index = index / (std::mem::size_of::<T>() * 8);
        let bit_index = index % (std::mem::size_of::<T>() * 8);

        if element_index >= N {
            panic!("Index out of bounds");
        }

        let mask = T::one() << bit_index;
        let was_set = (self.bits[element_index] & mask) != T::default();
        self.bits[element_index] |= mask;
        !was_set
    }

    fn remove(&mut self, index: usize) {
        let element_index = index / (std::mem::size_of::<T>() * 8);
        let bit_index = index % (std::mem::size_of::<T>() * 8);

        if element_index >= N {
            panic!("Index out of bounds");
        }

        let mask = !(T::one() << bit_index);
        self.bits[element_index] &= mask;
    }

    fn contains(&self, index: usize) -> bool {
        let element_index = index / (std::mem::size_of::<T>() * 8);
        let bit_index = index % (std::mem::size_of::<T>() * 8);

        if element_index >= N {
            panic!("Index out of bounds");
        }

        let mask = T::one() << bit_index;
        (self.bits[element_index] & mask) != T::default()
    }

    fn count(&self) -> usize {
        // More efficient counting using SIMD operations
        self.bits.to_array().iter().map(|&x| x.count_ones() as usize).sum()
    }
}

impl<T, const N: usize> BitsetRangeOps for SimdBitset<T, N>
where
    T: SimdElement + Default + Copy + Eq + One + PrimInt + Not<Output = T> + 
       BitAnd<Output = T> + BitAndAssign + BitOr<Output = T> + BitOrAssign,
    LaneCount<N>: SupportedLaneCount,
{
    fn insert_range<R: RangeBounds<usize>>(&mut self, range: R) {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => Self::fixed_capacity(),
        };

        let bits_per_element = std::mem::size_of::<T>() * 8;

        // Fast path: if we're setting a range of full elements
        if start % bits_per_element == 0 && end % bits_per_element == 0 {
            let start_element = start / bits_per_element;
            let end_element = end / bits_per_element;
            
            for i in start_element..end_element {
                if i < N {
                    self.bits[i] = !T::default();
                }
            }
            return;
        }

        // Slow path: set individual bits
        for i in start..end {
            if i < Self::fixed_capacity() {
                self.insert(i);
            }
        }
    }

    fn remove_range<R: RangeBounds<usize>>(&mut self, range: R) {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => Self::fixed_capacity(),
        };

        let bits_per_element = std::mem::size_of::<T>() * 8;

        // Fast path: if we're clearing a range of full elements
        if start % bits_per_element == 0 && end % bits_per_element == 0 {
            let start_element = start / bits_per_element;
            let end_element = end / bits_per_element;
            
            for i in start_element..end_element {
                if i < N {
                    self.bits[i] = T::default();
                }
            }
            return;
        }

        // Slow path: clear individual bits
        for i in start..end {
            if i < Self::fixed_capacity() {
                self.remove(i);
            }
        }
    }
}

impl<T, const N: usize> BitsetOpsUnsafe for SimdBitset<T, N>
where
    T: SimdElement + Default + Copy + Eq + One + PrimInt + Not<Output = T> + 
       BitAnd<Output = T> + BitAndAssign + BitOr<Output = T> + BitOrAssign,
    LaneCount<N>: SupportedLaneCount,
{
    unsafe fn insert_unchecked(&mut self, index: usize) -> bool {
        let element_index = index / (std::mem::size_of::<T>() * 8);
        let bit_index = index % (std::mem::size_of::<T>() * 8);
        
        let mask = T::one() << bit_index;
        let was_set = (self.bits[element_index] & mask) != T::default();
        self.bits[element_index] |= mask;
        !was_set
    }

    unsafe fn remove_unchecked(&mut self, index: usize) {
        let element_index = index / (std::mem::size_of::<T>() * 8);
        let bit_index = index % (std::mem::size_of::<T>() * 8);
        
        let mask = !(T::one() << bit_index);
        self.bits[element_index] &= mask;
    }

    unsafe fn contains_unchecked(&self, index: usize) -> bool {
        let element_index = index / (std::mem::size_of::<T>() * 8);
        let bit_index = index % (std::mem::size_of::<T>() * 8);
        
        let mask = T::one() << bit_index;
        (self.bits[element_index] & mask) != T::default()
    }
}

pub struct SimdBitsetIterator<T, const N: usize>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    bitset: SimdBitset<T, N>,
    current_element: usize,
    current_bit: usize,
}

impl<T, const N: usize> Iterator for SimdBitsetIterator<T, N>
where
    T: SimdElement + Default + One + PartialEq + Shl<usize, Output = T> + BitAnd<Output = T>,
    LaneCount<N>: SupportedLaneCount,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let bits_per_element = std::mem::size_of::<T>() * 8;
        
        while self.current_element < N {
            let value = self.bitset.bits[self.current_element];
            
            // Skip over if entire element is 0
            if value == T::default() {
                self.current_element += 1;
                self.current_bit = 0;
                continue;
            }
            
            // Find next set bit
            while self.current_bit < bits_per_element {
                let mask = T::one() << self.current_bit;
                if (value & mask) != T::default() {
                    let result = self.current_element * bits_per_element + self.current_bit;
                    self.current_bit += 1;
                    return Some(result);
                }
                self.current_bit += 1;
            }
            
            // Move to next element
            self.current_element += 1;
            self.current_bit = 0;
        }
        
        None
    }
}

impl<T, const N: usize> DoubleEndedIterator for SimdBitsetIterator<T, N>
where
    T: SimdElement + WrappingSub + BitAndAssign + One + Default + Copy + Eq + PrimInt + Not<Output = T> + 
       BitAnd<Output = T> + BitAndAssign + BitOr<Output = T> + BitOrAssign,
    LaneCount<N>: SupportedLaneCount,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let bits_per_element = std::mem::size_of::<T>() * 8;
        
        let mut element_index = N;
        while element_index > 0 {
            element_index -= 1;
            
            let value = self.bitset.bits[element_index];
            
            // Skip over if entire element is 0
            if value == T::default() {
                continue;
            }
            
            // Find the highest set bit in this element
            let mut bit_index = bits_per_element;
            while bit_index > 0 {
                bit_index -= 1;
                
                let mask = T::one() << bit_index;
                if (value & mask) != T::default() {
                    let result = element_index * bits_per_element + bit_index;
                    
                    // Create a copy of the bitset with this bit unset
                    let mut new_bitset = self.bitset.clone();
                    new_bitset.remove(result);
                    self.bitset = new_bitset;
                    
                    return Some(result);
                }
            }
        }
        
        None
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a SimdBitset<T, N>
where
    T: SimdElement + WrappingSub + BitAndAssign + One + Default + Copy + Eq + PrimInt + Not<Output = T> + 
       BitAnd<Output = T> + BitAndAssign + BitOr<Output = T> + BitOrAssign,
    LaneCount<N>: SupportedLaneCount,
{
    type IntoIter = SimdBitsetIterator<T, N>;
    type Item = usize;

    fn into_iter(self) -> Self::IntoIter {
        SimdBitsetIterator {
            bitset: self.clone(),
            current_element: 0,
            current_bit: 0,
        }
    }
}

// Define common SIMD bitset types with supported lane counts
pub type SimdU8Bitset2 = SimdBitset<u8, 2>;
pub type SimdU8Bitset4 = SimdBitset<u8, 4>;
pub type SimdU8Bitset8 = SimdBitset<u8, 8>;
pub type SimdU8Bitset16 = SimdBitset<u8, 16>;
pub type SimdU8Bitset32 = SimdBitset<u8, 32>;

pub type SimdU16Bitset2 = SimdBitset<u16, 2>;
pub type SimdU16Bitset4 = SimdBitset<u16, 4>;
pub type SimdU16Bitset8 = SimdBitset<u16, 8>;
pub type SimdU16Bitset16 = SimdBitset<u16, 16>;

pub type SimdU32Bitset2 = SimdBitset<u32, 2>;
pub type SimdU32Bitset4 = SimdBitset<u32, 4>;
pub type SimdU32Bitset8 = SimdBitset<u32, 8>;

pub type SimdU64Bitset2 = SimdBitset<u64, 2>;
pub type SimdU64Bitset4 = SimdBitset<u64, 4>;

#[cfg(test)]
mod tests {
    use super::super::tests::*;
    use super::*;
    
    // Using fixed types with supported lane counts
    crate::generate_tests!(test_empty, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_full, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_set_get, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_unset, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_set_range, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_set_unset_get, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_unset_range, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_set_all, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_bitwise_and, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_bitwise_and_assign, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_bitwise_or, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_bitwise_or_assign, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_empty_iterator, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_empty_iterator_back, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_set_one_bit_iterator, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_one_bit_iterator_back, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_set_two_bit_iterator, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
    crate::generate_tests!(test_set_two_bit_iterator_back, SimdU8Bitset8, SimdU16Bitset8, SimdU32Bitset8, SimdU64Bitset4);
}