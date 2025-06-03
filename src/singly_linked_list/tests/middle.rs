use super::*;

#[test]
fn test_0() {
    let l = SinglyLinkedList::<i32>::new();
    assert!(l.middle().is_none());
}

#[test]
fn test_1() {
    let mut l = SinglyLinkedList::new();
    l.push_back(1);
    assert_eq!(l.middle().unwrap().value, 1);
}

#[test]
fn test_2() {
    let mut l = SinglyLinkedList::new();
    l.push_back(1);
    l.push_back(2);
    assert_eq!(l.middle().unwrap().value, 2);
}

#[test]
fn test_3() {
    let mut l = SinglyLinkedList::new();
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    assert_eq!(l.middle().unwrap().value, 2);
}

#[test]
fn test_4() {
    let mut l = SinglyLinkedList::new();
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    l.push_back(4);
    assert_eq!(l.middle().unwrap().value, 3);
}

#[test]
fn test_5() {
    let mut l = SinglyLinkedList::new();
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    l.push_back(4);
    l.push_back(5);
    assert_eq!(l.middle().unwrap().value, 3);
}
