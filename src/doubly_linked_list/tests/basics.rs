use super::*;

#[test]
fn test_push_pop_front() {
    let mut list = List::new();
    assert!(list.is_empty());
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front().unwrap(), 3);
    assert_eq!(list.pop_front().unwrap(), 2);
    assert_eq!(list.pop_front().unwrap(), 1);
    assert!(list.pop_front().is_none());
    assert!(list.is_empty());
}

#[test]
fn test_push_pop_back() {
    let mut list = List::new();
    assert!(list.is_empty());
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_back().unwrap(), 3);
    assert_eq!(list.pop_back().unwrap(), 2);
    assert_eq!(list.pop_back().unwrap(), 1);
    assert!(list.pop_back().is_none());
    assert!(list.is_empty());
}

#[test]
fn test_push_front_pop_back() {
    let mut list = List::new();
    list.push_front(1);
    list.push_front(2);
    assert_eq!(list.pop_back().unwrap(), 1);
    assert_eq!(list.pop_back().unwrap(), 2);
}

#[test]
fn test_clear() {
    static mut COUNT: i32 = 0;

    struct Foo(i32);

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Foo {} drop", self.0);
            unsafe {
                COUNT += 1;
            }
        }
    }

    let mut list = List::new();
    list.push_front(Foo(2));
    list.push_front(Foo(1));
    list.push_front(Foo(0));
    list.clear();
    unsafe {
        let count = COUNT;
        assert_eq!(count, 3);
    }
}

#[test]
fn test() {
    let mut list = List::new();
    assert!(list.is_empty());

    list.push_back(3);
    list.push_front(2);
    list.push_back(4);
    list.push_front(1);
    list.push_back(5);
    list.push_front(0);
    assert_eq!(list.len(), 6);

    assert_eq!(list.pop_front().unwrap(), 0);
    assert_eq!(list.pop_back().unwrap(), 5);
    assert_eq!(list.pop_front().unwrap(), 1);
    assert_eq!(list.pop_back().unwrap(), 4);
    assert_eq!(list.pop_front().unwrap(), 2);
    assert_eq!(list.pop_back().unwrap(), 3);
    assert!(list.is_empty());
    assert!(list.pop_front().is_none());
    assert!(list.pop_back().is_none());
}

#[test]
fn basics() {
    let mut list = List::new();

    assert_eq!(list.pop_front(), None);

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));

    list.push_front(4);
    list.push_front(5);

    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
}
