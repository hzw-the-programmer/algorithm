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
