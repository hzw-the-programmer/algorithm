use super::*;

#[test]
fn test_push_pop_front() {
    let mut dll = DoublyLinkedList::new();
    assert!(dll.is_empty());
    dll.push_front(1);
    dll.push_front(2);
    dll.push_front(3);
    assert_eq!(dll.len(), 3);
    assert_eq!(dll.pop_front().unwrap(), 3);
    assert_eq!(dll.pop_front().unwrap(), 2);
    assert_eq!(dll.pop_front().unwrap(), 1);
    assert!(dll.pop_front().is_none());
    assert!(dll.is_empty());
}
