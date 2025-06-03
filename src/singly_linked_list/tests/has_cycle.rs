use super::*;

#[test]
fn test_1() {
    let n4 = Node::new_box(4, None);
    let n4_raw = Box::into_raw(n4);
    let n4 = unsafe { Box::from_raw(n4_raw) };

    let n3 = Node::new_link(3, Some(n4));

    let mut n2 = Node::new_box(2, n3);
    let n2_raw = Box::into_raw(n2);
    n2 = unsafe { Box::from_raw(n2_raw) };

    let n1 = Node::new_link(1, Some(n2));
    let n0 = Node::new_link(0, n1);

    let mut l = SinglyLinkedList::new();
    l.head = n0;

    unsafe {
        (*n4_raw).next = Some(Box::from_raw(n2_raw));
    }

    assert!(l.has_cycle());

    unsafe {
        let _ = std::mem::ManuallyDrop::new((*n4_raw).next.take());
    }
}

#[test]
fn test_2() {
    let mut l = SinglyLinkedList::new();
    l.push_back(0);
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    l.push_back(4);
    assert!(!l.has_cycle());
}
