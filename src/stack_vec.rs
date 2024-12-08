use std::fmt::{Debug, Formatter};
use std::slice::Iter;

pub struct ArrayVec<T, const N: usize> {
    len: usize,
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> ArrayVec<T, N> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
            len: 0,
        }
    }

    pub fn zeros(len: usize) -> Self {
        Self {
            data: [T::default(); N],
            len: len,
        }
    }
}

#[allow(dead_code)]
impl<T, const N: usize> ArrayVec<T, N> {
    pub unsafe fn clear(&mut self) {
        self.len = 0;
    }

    pub unsafe fn push_unchecked(&mut self, value: T) {
        *self.data.get_unchecked_mut(self.len) = value;
        self.len += 1;
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        &self.data.get_unchecked(index)
    }

    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.data.get_unchecked_mut(index)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data[..self.len()].iter()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Debug, const N: usize> Debug for ArrayVec<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
