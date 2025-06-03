use super::*;

#[test]
fn test() {
    let mut l = SinglyLinkedList::new();

    assert_eq!(l.len(), 0);
    assert!(l.is_empty());
    assert!(l.pop_front().is_none());

    l.push_front(1);
    assert_eq!(l.len(), 1);
    assert!(!l.is_empty());
    assert_eq!(l.pop_front().unwrap(), 1);
    assert_eq!(l.len(), 0);
    assert!(l.is_empty());
    assert!(l.pop_front().is_none());

    l.push_back(1);
    assert_eq!(l.len(), 1);
    assert!(!l.is_empty());
    assert_eq!(l.pop_front().unwrap(), 1);
    assert_eq!(l.len(), 0);
    assert!(l.is_empty());
    assert!(l.pop_front().is_none());

    l.push_back(1);
    l.push_front(2);
    assert_eq!(l.len(), 2);
    assert!(!l.is_empty());

    l.clear();
    assert_eq!(l.len(), 0);
    assert!(l.is_empty());
}
