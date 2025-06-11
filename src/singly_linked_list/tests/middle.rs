use super::*;

#[test]
fn test_0() {
    let list = List::<i32>::new();
    assert_eq!(list.middle(), None);
}

#[test]
fn test_1() {
    let mut list = List::new();
    list.push_back(1);
    assert_eq!(list.middle(), Some(&1));
}

#[test]
fn test_2() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    assert_eq!(list.middle(), Some(&2));
}

#[test]
fn test_3() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.middle(), Some(&2));
}

#[test]
fn test_4() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    assert_eq!(list.middle(), Some(&3));
}

#[test]
fn test_5() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    assert_eq!(list.middle(), Some(&3));
}
