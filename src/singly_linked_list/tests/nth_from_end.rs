use super::*;

#[test]
fn test() {
    let mut l = SinglyLinkedList::new();

    l.push_back(1);
    assert_eq!(l.nth_from_end(1).unwrap().value, 1);
    assert!(l.nth_from_end(2).is_none());

    l.push_back(2);
    assert_eq!(l.nth_from_end(1).unwrap().value, 2);
    assert_eq!(l.nth_from_end(2).unwrap().value, 1);
    assert!(l.nth_from_end(3).is_none());

    l.push_back(3);
    assert_eq!(l.nth_from_end(1).unwrap().value, 3);
    assert_eq!(l.nth_from_end(2).unwrap().value, 2);
    assert_eq!(l.nth_from_end(3).unwrap().value, 1);
    assert!(l.nth_from_end(4).is_none());
}
