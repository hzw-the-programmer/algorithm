use super::*;
use core::ops::{Deref, DerefMut};

pub struct Stack<T>(List<T>);

impl<T> Deref for Stack<T> {
    type Target = List<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Stack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack(List::new())
    }

    pub fn push(&mut self, value: T) {
        self.push_front(value);
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

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
