use super::*;

#[test]
fn test_insert() {
    let mut t = AVLTree::new();
    t.insert(10);
    // assert_ne!(t, bt::btree![1]);
    // assert_eq!(t, bt::btree![10]);
    t.insert(9);
}
