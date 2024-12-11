use std::marker::PhantomData;

use num::PrimInt;

use crate::stack_vec::ArrayVec;

use super::{primitives::PrimitiveBitset, BitsetOps, FixedSizeBitset};

pub struct SparseBitset<C, U> {
    components: C,
    _phantom: PhantomData<U>,
}

trait Components<U>
where
    PrimitiveBitset<U>: FixedSizeBitset,
{
    fn index_offset(&self, value: usize) -> (usize, usize) {
        (
            value / PrimitiveBitset::<U>::fixed_capacity(),
            value % PrimitiveBitset::<U>::fixed_capacity(),
        )
    }

    fn empty() -> Self;

    fn as_slice(&self) -> &[SparseEntry<U>];

    fn as_mut_slice(&mut self) -> &mut [SparseEntry<U>];

    fn push_component(&mut self, index: usize, offset: usize);
}

struct SparseEntry<U> {
    index: usize,
    bits: PrimitiveBitset<U>,
}

impl<U: PrimInt, const N: usize> Components<U> for ArrayVec<SparseEntry<U>, N> {
    fn empty() -> Self {
        ArrayVec::new()
    }

    fn as_slice(&self) -> &[SparseEntry<U>] {
        self.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [SparseEntry<U>] {
        self.as_mut_slice()
    }

    fn push_component(&mut self, index: usize, offset: usize) {
        let mut bits = PrimitiveBitset::<U>::empty();
        bits.set(offset);
        unsafe { self.push_unchecked(SparseEntry { index, bits }) };
    }
}

impl<U: PrimInt> Components<U> for Vec<SparseEntry<U>> {
    fn empty() -> Self {
        Vec::new()
    }

    fn as_slice(&self) -> &[SparseEntry<U>] {
        self.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [SparseEntry<U>] {
        self.as_mut_slice()
    }

    fn push_component(&mut self, index: usize, offset: usize) {
        let mut bits = PrimitiveBitset::<U>::empty();
        bits.set(offset);
        self.push(SparseEntry { index, bits });
    }
}

impl<C: Components<U>, U: PrimInt> BitsetOps for SparseBitset<C, U> {
    fn empty() -> Self {
        SparseBitset {
            components: C::empty(),
            _phantom: PhantomData,
        }
    }

    fn set(&mut self, value: usize) -> bool {
        let (index, offset) = self.components.index_offset(value);
        for SparseEntry { index: idx, bits } in self.components.as_mut_slice() {
            if *idx == index {
                return bits.set(offset);
            }
        }

        self.components.push_component(index, offset);
        true
    }

    fn unset(&mut self, value: usize) {
        let (index, offset) = self.components.index_offset(value);
        for SparseEntry { index: idx, bits } in self.components.as_mut_slice() {
            if *idx == index {
                return bits.unset(offset);
            }
        }
    }

    fn get(&self, value: usize) -> bool {
        let (index, offset) = self.components.index_offset(value);
        for bits in self.components.as_slice() {
            if bits.index == index {
                return bits.bits.get(offset);
            }
        }
        false
    }

    fn count(&self) -> usize {
        self.components
            .as_slice()
            .iter()
            .map(|bits| bits.bits.count())
            .sum()
    }
}
