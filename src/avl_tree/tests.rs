use super::*;
use crate::binary_tree as bt;

#[test]
fn test_ll_insert() {
    let mut t = AVLTree::new();

    t.insert(10);
    assert_eq!(t, bt::btree![10]);

    t.insert(9);
    assert_eq!(t, bt::btree![10, 9]);

    t.insert(8);
    assert_ne!(t, bt::btree![10, 9, null, 8]);
    assert_eq!(t, bt::btree![9, 8, 10]);

    t.insert(7);
    assert_eq!(t, bt::btree![9, 8, 10, 7, null]);

    t.insert(6);
    assert_ne!(t, bt::btree![9, 8, 10, 7, null, null, null, 6]);
    assert_eq!(t, bt::btree![9, 7, 10, 6, 8]);

    t.insert(5);
    assert_ne!(t, bt::btree![9, 7, 10, 6, 8, null, null, 5]);
    assert_eq!(t, bt::btree![7, 6, 9, 5, null, 8, 10]);

    t.insert(4);
    assert_ne!(t, bt::btree![7, 6, 9, 5, null, 8, 10, 4]);
    assert_eq!(t, bt::btree![7, 5, 9, 4, 6, 8, 10]);

    t.insert(3);
    assert_eq!(t, bt::btree![7, 5, 9, 4, 6, 8, 10, 3]);

    t.insert(2);
    assert_ne!(
        t,
        bt::btree![
            7, 5, 9, 4, 6, 8, 10, 3, null, null, null, null, null, null, null, 2
        ]
    );
    assert_eq!(t, bt::btree![7, 5, 9, 3, 6, 8, 10, 2, 4]);

    t.insert(1);
    assert_ne!(
        t,
        bt::btree![
            7, 5, 9, 3, 6, 8, 10, 2, 4, null, null, null, null, null, null, 1
        ]
    );
    assert_eq!(t, bt::btree![7, 3, 9, 2, 5, 8, 10, 1, null, 4, 6]);
}

#[test]
fn test_rr_insert() {
    let mut t = AVLTree::new();

    t.insert(0);
    assert_eq!(t, bt::btree![0]);

    t.insert(1);
    assert_eq!(t, bt::btree![0, null, 1]);

    t.insert(2);
    assert_ne!(t, bt::btree![0, null, 1, null, null, null, 2]);
    assert_eq!(t, bt::btree![1, 0, 2]);

    t.insert(3);
    assert_eq!(t, bt::btree![1, 0, 2, null, null, null, 3]);

    t.insert(4);
    assert_eq!(t, bt::btree![1, 0, 3, null, null, 2, 4]);

    t.insert(5);
    assert_eq!(t, bt::btree![3, 1, 4, 0, 2, null, 5]);

    t.insert(6);
    assert_eq!(t, bt::btree![3, 1, 5, 0, 2, 4, 6]);

    t.insert(7);
    assert_eq!(
        t,
        bt::btree![
            3, 1, 5, 0, 2, 4, 6, null, null, null, null, null, null, null, 7
        ]
    );

    t.insert(8);
    assert_eq!(
        t,
        bt::btree![
            3, 1, 5, 0, 2, 4, 7, null, null, null, null, null, null, 6, 8
        ]
    );

    t.insert(9);
    assert_eq!(
        t,
        bt::btree![3, 1, 7, 0, 2, 5, 8, null, null, null, null, 4, 6, null, 9]
    );
}

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
