use super::*;

#[test]
fn test_1() {
    let n4 = Node::new_box(4, None);
    let n4_raw = Box::into_raw(n4);
    let n4 = unsafe { Box::from_raw(n4_raw) };

    let n3 = Node::new_link(3, Some(n4));

    let mut n2 = Node::new_box(2, n3);
    println!("n2.1 {:p}", n2);
    let n2_raw = Box::into_raw(n2);
    println!("n2.2 {:p}", n2_raw);
    n2 = unsafe { Box::from_raw(n2_raw) };
    println!("n2.3 {:p}", n2);
    // let a = Some(n2);
    // println!("n2.4 {:p}", a.unwrap());
    // println!("n2.5 {:p}", a.as_ref().unwrap());
    // println!("n2.6 {:p}", *a.as_ref().unwrap());
    // let i: i32 = *a.as_ref().unwrap();
    // let i: i32 = &*a.as_ref().unwrap();
    // let i: i32 = &**a.as_ref().unwrap();
    // println!("n2.7 {:p}", a.as_deref().unwrap());

    let n1 = Node::new_link(1, Some(n2));
    let n0 = Node::new_link(0, n1);

    let mut l = SinglyLinkedList::new();
    l.head = n0;

    unsafe {
        (*n4_raw).next = Some(Box::from_raw(n2_raw));
        println!("n4.1 = {:p}", *(*n4_raw).next.as_ref().unwrap());
    }

    assert_eq!(l.cycle_entry().unwrap().value, 2);
    // assert!(l.cycle_entry().is_none());

    unsafe {
        let _ = std::mem::ManuallyDrop::new((*n4_raw).next.take());
    }
}

// #[test]
fn test_2() {
    let mut l = SinglyLinkedList::new();
    l.push_back(0);
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    l.push_back(4);
    assert!(l.cycle_entry().is_none());
}
