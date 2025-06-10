use super::*;

pub struct IntoIter<T>(SinglyLinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> IntoIterator for SinglyLinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = <IntoIter<T> as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}
