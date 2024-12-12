use std::fmt::{Debug, Formatter};
use std::mem::MaybeUninit;
use std::slice::Iter;

#[derive(Clone, Copy)]
pub struct ArrayVec<T, const N: usize> {
    len: usize,
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> ArrayVec<T, N> {
    pub fn zeros(len: usize) -> Self {
        Self {
            data: [T::default(); N],
            len: len,
        }
    }
}

impl<T: Copy, const N: usize> ArrayVec<T, N> {
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(*self.data.get_unchecked(self.len)) }
        }
    }

    pub unsafe fn pop_unsafe(&mut self) -> T {
        self.len -= 1;
        *self.data.get_unchecked(self.len)
    }
}

impl<T, const N: usize> ArrayVec<T, N> {
    pub fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
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

    pub fn get_last(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            unsafe { Some(self.data.get_unchecked(self.len - 1)) }
        }
    }

    pub fn get_last_mut(&mut self) -> Option<&mut T> {
        if self.len == 0 {
            None
        } else {
            unsafe { Some(self.data.get_unchecked_mut(self.len - 1)) }
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data[..self.len()].iter()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data[..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[..self.len]
    }
}

impl<T: PartialEq, const N: usize> ArrayVec<T, N> {
    pub fn contains(&self, value: &T) -> bool {
        self.iter().any(|v| v == value)
    }
}

impl<T: Debug, const N: usize> Debug for ArrayVec<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a ArrayVec<T, N> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data[..self.len].iter()
    }
}
