use super::*;

#[test]
fn basics() {
    let mut stack = Stack::new();

    // Check empty stack behaves right
    assert_eq!(stack.pop(), None);

    // Populate stack
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Check normal removal
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    stack.push(4);
    stack.push(5);

    // Check normal removal
    assert_eq!(stack.pop(), Some(5));
    assert_eq!(stack.pop(), Some(4));

    // Check exhaustion
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}

#[test]
fn peek() {
    let mut stack = Stack::new();
    assert_eq!(stack.peek(), None);
    assert_eq!(stack.peek_mut(), None);
    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.peek(), Some(&3));
    assert_eq!(stack.peek_mut(), Some(&mut 3));

    stack.peek_mut().map(|value| *value = 42);

    assert_eq!(stack.peek(), Some(&42));
    assert_eq!(stack.pop(), Some(42));
}

#[test]
fn into_iter() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    let mut iter = stack.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    let mut iter = stack.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

#[test]
fn iter_mut() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    let mut iter = stack.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
}
