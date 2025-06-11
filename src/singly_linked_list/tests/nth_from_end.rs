use super::*;

#[test]
fn test() {
    let mut list = List::new();

    list.push_back(1);
    assert_eq!(list.nth_from_end(1), Some(&1));
    assert_eq!(list.nth_from_end(2), None);

    list.push_back(2);
    assert_eq!(list.nth_from_end(1), Some(&2));
    assert_eq!(list.nth_from_end(2), Some(&1));
    assert_eq!(list.nth_from_end(3), None);

    list.push_back(3);
    assert_eq!(list.nth_from_end(1), Some(&3));
    assert_eq!(list.nth_from_end(2), Some(&2));
    assert_eq!(list.nth_from_end(3), Some(&1));
    assert_eq!(list.nth_from_end(4), None);
}
