pub struct StackVec<T, const N: usize> {
    len: usize,
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> StackVec<T, N> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
            len: 0,
        }
    }
}

#[allow(dead_code)]
impl<T, const N: usize> StackVec<T, N> {
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

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().take(self.len)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
