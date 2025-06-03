use super::*;

#[test]
fn test() {
    let mut l = SinglyLinkedList::new();
    l.push_back(1);
    l.push_front(2);
    l.push_back(3);
    assert_eq!(l.iter().collect::<Vec<&i32>>(), vec![&2, &1, &3]);
}

#[test]
fn test_2() {
    let mut l = SinglyLinkedList::<i32>::new();
    l.push_back(1);
    l.push_front(2);
    l.push_back(3);
    let mut v = vec![];
    for n in &l {
        v.push(n);
    }
    assert_eq!(v, vec![&2, &1, &3]);
}
