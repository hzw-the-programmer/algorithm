use super::*;

impl<T> IntoIterator for SinglyLinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = <IntoIter<T> as Iterator>::Item;

    fn into_iter(mut self) -> Self::IntoIter {
        IntoIter {
            next: self.head.take(),
        }
    }
}

pub struct IntoIter<T> {
    next: Link<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next;
            node.value
        })
    }
}
