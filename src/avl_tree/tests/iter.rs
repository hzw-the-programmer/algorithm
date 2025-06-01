use super::*;

#[test]
fn test_1() {
    let mut t = AVLTree::new();
    t.insert(1);
    t.insert(2);
    assert_eq!(t.iter().collect::<Vec<&i32>>(), vec![&1, &2]);
}

#[test]
fn test_2() {
    let mut t = AVLTree::new();
    t.insert(1);
    t.insert(2);
    t.insert(3);
    assert_eq!(t, bt::btree![2, 1, 3]);
    assert_eq!(t.iter().collect::<Vec<&i32>>(), vec![&1, &2, &3]);
    assert_eq!(t, bt::btree![2, 1, 3]);
}

#[test]
fn test_3() {
    let mut t = AVLTree::new();
    t.insert(1);
    t.insert(2);
    t.insert(3);
    assert_eq!(t.iter().next(), Some(&1));
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
        v.push(n);
    }
    assert_eq!(v, [&1, &2, &3]);
    assert_eq!(t, bt::btree![2, 1, 3]);
}

#[test]
fn test_5() {
    let mut t = AVLTree::new();
    t.insert(1);
    t.insert(2);
    t.insert(3);
    let mut v = vec![];
    for n in &t {
        v.push(n);
        if v.len() == 1 {
            break;
        }
    }
    assert_eq!(v, [&1]);
    assert_eq!(t, bt::btree![2, 1, 3]);
}

#[test]
fn test_6() {
    let mut t = AVLTree::new();
    t.insert(1);
    t.insert(2);
    t.insert(3);
    let mut v = vec![];
    for n in &t {
        v.push(n);
        if v.len() == 2 {
            break;
        }
    }
    assert_eq!(v, [&1, &2]);
    assert_eq!(t, bt::btree![2, 1, 3]);
}
