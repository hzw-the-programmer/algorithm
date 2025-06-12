use super::*;
use core::ops::{Index, IndexMut};

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        self.get(idx).unwrap()
    }
}

impl<T> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.get_mut(idx).unwrap()
    }
}
