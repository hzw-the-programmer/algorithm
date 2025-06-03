use super::*;

#[test]
fn test() {
    let mut l = SinglyLinkedList::new();
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    let mut v = vec![];
    for n in l {
        v.push(n);
    }
    assert_eq!(v, vec![1, 2, 3]);
}
