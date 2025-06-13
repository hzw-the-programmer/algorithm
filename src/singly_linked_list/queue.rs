use super::*;
use core::ops::{Deref, DerefMut};

pub struct Queue<T>(List<T>);

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue(List::new())
    }

    pub fn push(&mut self, value: T) {
        self.push_back(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.peek_front()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.peek_front_mut()
    }
}

impl<T> Deref for Queue<T> {
    type Target = List<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Queue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
