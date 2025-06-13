use super::*;

#[test]
fn index() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list[0], 1);
    assert_eq!(list[1], 2);
    assert_eq!(list[2], 3);
}

#[test]
#[should_panic]
fn index_2() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list[0], 1);
    assert_eq!(list[1], 2);
    assert_eq!(list[2], 3);
    assert_eq!(list[3], 4);
}

#[test]
fn index_mut() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list[0] = 2;
    list[1] = 3;
    list[2] = 4;
    assert_eq!(list[0], 2);
    assert_eq!(list[1], 3);
    assert_eq!(list[2], 4);
}

#[test]
#[should_panic]
fn index_mut_2() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list[0] = 2;
    list[1] = 3;
    list[2] = 4;
    list[3] = 5;
}
