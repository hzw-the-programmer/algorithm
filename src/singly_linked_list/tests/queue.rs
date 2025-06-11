use super::*;

#[test]
fn basics() {
    let mut queue = Queue::new();

    // Check empty queue behaves right
    assert_eq!(queue.pop(), None);

    // Populate queue
    queue.push(1);
    queue.push(2);
    queue.push(3);

    // Check normal removal
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    queue.push(4);
    queue.push(5);

    // Check normal removal
    assert_eq!(queue.pop(), Some(3));
    assert_eq!(queue.pop(), Some(4));

    // Check exhaustion
    assert_eq!(queue.pop(), Some(5));
    assert_eq!(queue.pop(), None);

    // Check the exhaustion case fixed the pointer right
    queue.push(6);
    queue.push(7);

    // Check normal removal
    assert_eq!(queue.pop(), Some(6));
    assert_eq!(queue.pop(), Some(7));
    assert_eq!(queue.pop(), None);
}

#[test]
fn into_iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_mut() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), None);
}

#[test]
fn miri_food() {
    let mut list = List::new();

    list.push(1);
    list.push(2);
    list.push(3);

    assert!(list.pop() == Some(1));
    list.push(4);
    assert!(list.pop() == Some(2));
    list.push(5);

    assert!(list.peek() == Some(&3));
    list.push(6);
    list.peek_mut().map(|x| *x *= 10);
    assert!(list.peek() == Some(&30));
    assert!(list.pop() == Some(30));

    for elem in list.iter_mut() {
        *elem *= 100;
    }

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&400));
    assert_eq!(iter.next(), Some(&500));
    assert_eq!(iter.next(), Some(&600));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    assert!(list.pop() == Some(400));
    list.peek_mut().map(|x| *x *= 10);
    assert!(list.peek() == Some(&5000));
    list.push(7);

    // Drop it on the ground and let the dtor exercise itself
}
