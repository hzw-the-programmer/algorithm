use super::*;

#[test]
fn test() {
    let mut t = AVLTree::new();
    t.insert(5);
    t.insert(2);
    t.insert(8);

    assert!(t.search(&5));
    assert!(t.search(&2));
    assert!(t.search(&8));
    assert!(!t.search(&1));
    assert!(!t.search(&3));
    assert!(!t.search(&9));
    assert!(!t.search(&7));
}
