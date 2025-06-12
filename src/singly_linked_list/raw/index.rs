use super::*;
use core::ops::Index;

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        self.get(idx).unwrap()
    }
}
