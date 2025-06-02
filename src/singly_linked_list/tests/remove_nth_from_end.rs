use super::*;

#[test]
fn test() {
    let mut l = SinglyLinkedList::new();
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    assert!(l.remove_nth_from_end(4).is_none());
    assert_eq!(l.remove_nth_from_end(3).unwrap(), 1);
    l.push_front(1);
    assert_eq!(l.remove_nth_from_end(2).unwrap(), 2);
    assert_eq!(l.remove_nth_from_end(1).unwrap(), 3);
    assert_eq!(l.remove_nth_from_end(1).unwrap(), 1);
    assert!(l.remove_nth_from_end(1).is_none());
}

#[test]
#[should_panic]
fn test_2() {
    let mut l = SinglyLinkedList::<i32>::new();
    l.remove_nth_from_end(0);
}
