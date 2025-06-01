use super::*;

#[test]
fn test_1() {
    let mut t = AVLTree::new();
    t.insert(1);
    t.insert(2);
    assert_eq!(
        t.iter().map(|n| n.borrow().value).collect::<Vec<i32>>(),
        vec![1, 2]
    );
}

#[test]
fn test_2() {
    let mut t = AVLTree::new();
    t.insert(1);
    t.insert(2);
    t.insert(3);
    assert_eq!(t, bt::btree![2, 1, 3]);
    assert_eq!(
        t.iter().map(|n| n.borrow().value).collect::<Vec<i32>>(),
        vec![1, 2, 3]
    );
    assert_eq!(t, bt::btree![2, 1, 3]);
}

#[test]
fn test_3() {
    let mut t = AVLTree::new();
    t.insert(1);
    t.insert(2);
    t.insert(3);
    assert_eq!(t, bt::btree![2, 1, 3]);
    assert_eq!(t.iter().map(|n| n.borrow().value).next(), Some(1));
    assert_eq!(t, bt::btree![2, 1, 3]);
}

#[test]
fn test_4() {
    let mut t = AVLTree::new();
    t.insert(1);
    t.insert(2);
    t.insert(3);
    let mut v = vec![];
    for n in &t {
        v.push(n.borrow().value);
    }
    assert_eq!(v, [1, 2, 3]);
}
