use super::*;
use crate::binary_tree as bt;

mod delete;
mod insert;

impl<T: PartialEq> PartialEq<bt::Tree<T>> for AVLTree<T> {
    fn eq(&self, other: &bt::Tree<T>) -> bool {
        equal_recursive(self.root(), other)
    }
}

fn equal_recursive<T: PartialEq>(t1: &Tree<T>, t2: &bt::Tree<T>) -> bool {
    match (t1, t2) {
        (Some(n1), Some(n2)) => {
            if n1.borrow().value() != n2.borrow().value() {
                false
            } else {
                equal_recursive(&n1.borrow().left(), &n2.borrow().left())
                    && equal_recursive(&n1.borrow().right(), &n2.borrow().right())
            }
        }
        (None, None) => true,
        _ => false,
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for AVLTree<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self.root())
    }
}
