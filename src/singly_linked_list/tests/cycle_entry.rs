use super::*;

#[test]
fn test_1() {
    let mut list = List::new();
    for i in 0..5 {
        list.push_back(i);
    }
    list.link(4, 2);
    assert_eq!(list.cycle_entry(), Some(&2));
    list.unlink(4, 2);
}

#[test]
fn test_2() {
    let mut list = List::new();
    list.push_back(0);
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    assert_eq!(list.cycle_entry(), None);
}

#[test]
fn test_3() {
    let mut list = List::new();
    for i in 0..4 {
        list.push_back(i);
    }
    list.link(3, 2);
    assert_eq!(list.cycle_entry(), Some(&2));
    list.unlink(3, 2);
}
