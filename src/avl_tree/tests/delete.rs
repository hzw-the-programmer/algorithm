use super::*;

#[test]
fn test_1() {
    let mut t = AVLTree::new();

    t.insert(5);
    t.insert(3);
    t.insert(8);
    t.insert(1);
    t.insert(4);
    assert_eq!(t, bt::btree![5, 3, 8, 1, 4]);

    t.delete(&1);
    assert_eq!(t, bt::btree![5, 3, 8, null, 4]);

    t.delete(&3);
    assert_eq!(t, bt::btree![5, 4, 8]);
}

#[test]
fn test_2() {
    let mut t = AVLTree::new();

    t.insert(5);
    t.insert(3);
    t.insert(8);
    t.insert(1);
    t.insert(4);
    assert_eq!(t, bt::btree![5, 3, 8, 1, 4]);

    t.delete(&4);
    assert_eq!(t, bt::btree![5, 3, 8, 1]);

    t.delete(&3);
    assert_eq!(t, bt::btree![5, 1, 8]);
}

#[test]
fn test_3() {
    let mut t = AVLTree::new();

    t.insert(5);
    t.insert(3);
    t.insert(8);
    t.insert(1);
    t.insert(4);
    assert_eq!(t, bt::btree![5, 3, 8, 1, 4]);

    t.delete(&3);
    assert_eq!(t, bt::btree![5, 4, 8, 1]);
}

#[test]
fn test_ll() {
    let mut t = AVLTree::new();

    t.insert(5);
    t.insert(3);
    t.insert(8);
    t.insert(1);
    assert_eq!(t, bt::btree![5, 3, 8, 1]);

    t.delete(&8);
    assert_ne!(t, bt::btree![5, 3, null, 1]);
    assert_eq!(t, bt::btree![3, 1, 5]);
}

#[test]
fn test_ll_2() {
    let mut t = AVLTree::new();

    t.insert(5);
    t.insert(3);
    t.insert(8);
    t.insert(1);
    t.insert(4);
    assert_eq!(t, bt::btree![5, 3, 8, 1, 4]);

    t.delete(&8);
    assert_ne!(t, bt::btree![5, 3, null, 1, 4]);
    assert_eq!(t, bt::btree![3, 1, 5, null, null, 4]);
}

#[test]
fn test_lr() {
    let mut t = AVLTree::new();

    t.insert(5);
    t.insert(3);
    t.insert(8);
    t.insert(4);
    assert_eq!(t, bt::btree![5, 3, 8, null, 4]);

    t.delete(&8);
    assert_ne!(t, bt::btree![5, 3, null, null, 4]);
    assert_eq!(t, bt::btree![4, 3, 5]);
}

#[test]
fn test_rr() {
    let mut t = AVLTree::new();

    t.insert(3);
    t.insert(2);
    t.insert(5);
    t.insert(8);
    assert_eq!(t, bt::btree![3, 2, 5, null, null, null, 8]);

    t.delete(&2);
    assert_ne!(t, bt::btree![3, null, 5, null, null, null, 8]);
    assert_eq!(t, bt::btree![5, 3, 8]);
}

#[test]
fn test_rr_2() {
    let mut t = AVLTree::new();

    t.insert(3);
    t.insert(2);
    t.insert(5);
    t.insert(8);
    t.insert(4);
    assert_eq!(t, bt::btree![3, 2, 5, null, null, 4, 8]);

    t.delete(&2);
    assert_ne!(t, bt::btree![3, null, 5, null, null, 4, 8]);
    assert_eq!(t, bt::btree![5, 3, 8, null, 4]);
}

#[test]
fn test_rl() {
    let mut t = AVLTree::new();

    t.insert(3);
    t.insert(2);
    t.insert(5);
    t.insert(4);
    assert_eq!(t, bt::btree![3, 2, 5, null, null, 4]);

    t.delete(&2);
    assert_ne!(t, bt::btree![3, 2, 5, null, null, 4]);
    assert_eq!(t, bt::btree![4, 3, 5]);
}
