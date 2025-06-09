use super::*;

#[test]
fn test() {
    let mut l = SinglyLinkedList::new();
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    for n in &mut l {
        *n += 1;
    }
    assert_eq!(l.iter().collect::<Vec<&i32>>(), vec![&2, &3, &4]);
}

#[test]
fn iter_mut() {
    let mut list = SinglyLinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), None);
}
