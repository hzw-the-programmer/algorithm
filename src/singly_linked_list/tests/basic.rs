use super::*;

#[test]
fn test_pop_front() {
    let mut l = SinglyLinkedList::new();
    assert!(l.pop_front().is_none());
    l.push_front(1);
    assert_eq!(l.pop_front().unwrap(), 1);
    l.push_back(1);
    assert_eq!(l.pop_front().unwrap(), 1);
    assert!(l.pop_front().is_none());
}
