use super::*;
use core::ops::{Deref, DerefMut};

pub struct Queue<T>(List<T>);

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue(List::new())
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
