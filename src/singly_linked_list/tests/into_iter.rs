use super::*;

#[test]
fn test() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    let mut v = vec![];
    for n in list {
        v.push(n);
    }
    assert_eq!(v, vec![1, 2, 3]);
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
fn into_iter_2() {
    let mut list = List::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}
