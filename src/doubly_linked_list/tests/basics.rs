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

#[test]
fn test_push_pop_back() {
    let mut dll = DoublyLinkedList::new();
    assert!(dll.is_empty());
    dll.push_back(1);
    dll.push_back(2);
    dll.push_back(3);
    assert_eq!(dll.len(), 3);
    assert_eq!(dll.pop_back().unwrap(), 3);
    assert_eq!(dll.pop_back().unwrap(), 2);
    assert_eq!(dll.pop_back().unwrap(), 1);
    assert!(dll.pop_back().is_none());
    assert!(dll.is_empty());
}

#[test]
fn test_push_front_pop_back() {
    let mut dll = DoublyLinkedList::new();
    dll.push_front(1);
    dll.push_front(2);
    assert_eq!(dll.pop_back().unwrap(), 1);
    assert_eq!(dll.pop_back().unwrap(), 2);
}

#[test]
fn test() {
    let mut dll = DoublyLinkedList::new();
    assert!(dll.is_empty());

    dll.push_back(3);
    dll.push_front(2);
    dll.push_back(4);
    dll.push_front(1);
    dll.push_back(5);
    dll.push_front(0);
    assert_eq!(dll.len(), 6);

    assert_eq!(dll.pop_front().unwrap(), 0);
    assert_eq!(dll.pop_back().unwrap(), 5);
    assert_eq!(dll.pop_front().unwrap(), 1);
    assert_eq!(dll.pop_back().unwrap(), 4);
    assert_eq!(dll.pop_front().unwrap(), 2);
    assert_eq!(dll.pop_back().unwrap(), 3);
    assert!(dll.is_empty());
    assert!(dll.pop_front().is_none());
    assert!(dll.pop_back().is_none());
}
