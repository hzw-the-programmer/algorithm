use super::*;

#[test]
fn miri_food() {
    let mut list = List::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert!(list.pop_front() == Some(1));
    list.push_back(4);
    assert!(list.pop_front() == Some(2));
    list.push_back(5);

    assert!(list.peek_front() == Some(&3));
    list.push_back(6);
    list.peek_front_mut().map(|x| *x *= 10);
    assert!(list.peek_front() == Some(&30));
    assert!(list.pop_front() == Some(30));

    for elem in list.iter_mut() {
        *elem *= 100;
    }

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&400));
    assert_eq!(iter.next(), Some(&500));
    assert_eq!(iter.next(), Some(&600));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    assert!(list.pop_front() == Some(400));
    list.peek_front_mut().map(|x| *x *= 10);
    assert!(list.peek_front() == Some(&5000));
    list.push_back(7);

    // Drop it on the ground and let the dtor exercise itself
}
