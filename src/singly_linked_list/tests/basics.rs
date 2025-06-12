use super::*;

#[test]
fn test() {
    let mut list = List::new();

    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
    assert!(list.pop_front().is_none());

    list.push_front(1);
    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());
    assert_eq!(list.pop_front().unwrap(), 1);
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
    assert!(list.pop_front().is_none());

    list.push_back(1);
    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());
    assert_eq!(list.pop_front().unwrap(), 1);
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
    assert!(list.pop_front().is_none());

    list.push_back(1);
    list.push_front(2);
    assert_eq!(list.len(), 2);
    assert!(!list.is_empty());

    list.clear();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek_front(), None);
    assert_eq!(list.peek_front_mut(), None);

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    assert_eq!(list.peek_front(), Some(&3));
    assert_eq!(list.peek_front_mut(), Some(&mut 3));

    list.peek_front_mut().map(|v| *v = 42);
    assert_eq!(list.peek_front(), Some(&42));
    assert_eq!(list.peek_front_mut(), Some(&mut 42));
}

#[test]
fn basics() {
    let mut list = List::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), None);

    // Check the exhaustion case fixed the pointer right
    list.push(6);
    list.push(7);

    // Check normal removal
    assert_eq!(list.pop(), Some(6));
    assert_eq!(list.pop(), Some(7));
    assert_eq!(list.pop(), None);
}

#[test]
fn get() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.get(0), Some(&1));
    assert_eq!(list.get(1), Some(&2));
    assert_eq!(list.get(2), Some(&3));
    assert_eq!(list.get(3), None);
}

#[test]
fn get_mut() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.get_mut(0), Some(&mut 1));
    assert_eq!(list.get_mut(1), Some(&mut 2));
    assert_eq!(list.get_mut(2), Some(&mut 3));
    assert_eq!(list.get_mut(3), None);

    list.get_mut(1).map(|v| *v = 4);
    assert_eq!(list.get_mut(0), Some(&mut 1));
    assert_eq!(list.get_mut(1), Some(&mut 4));
    assert_eq!(list.get_mut(2), Some(&mut 3));
    assert_eq!(list.get_mut(3), None);
}
