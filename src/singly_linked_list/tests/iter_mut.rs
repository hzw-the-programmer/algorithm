use super::*;

#[test]
fn test() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    for n in &mut list {
        *n += 1;
    }
    assert_eq!(list.iter().collect::<Vec<&i32>>(), vec![&2, &3, &4]);
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
fn iter_mut_2() {
    let mut list = List::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), None);
}
