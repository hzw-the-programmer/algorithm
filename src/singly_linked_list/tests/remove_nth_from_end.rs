use super::*;

#[test]
fn test() {
    let mut list = List::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.remove_nth_from_end(4), None);
    assert_eq!(list.remove_nth_from_end(3), Some(1));

    list.push_front(1);
    assert_eq!(list.remove_nth_from_end(2), Some(2));
    assert_eq!(list.remove_nth_from_end(1), Some(3));
    assert_eq!(list.remove_nth_from_end(1), Some(1));
    assert_eq!(list.remove_nth_from_end(1), None);

    list.push_back(1);
    assert_eq!(list.len(), 1);
    assert_eq!(list.pop_front(), Some(1));

    list.push_back(1);
    list.push_back(2);
    assert_eq!(list.remove_nth_from_end(1), Some(2));
    list.push_back(3);
}
