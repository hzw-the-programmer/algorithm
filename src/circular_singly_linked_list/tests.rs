use super::*;

#[test]
fn test_push_pop_front() {}

#[test]
fn test_drop_1() {
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

    {
        let mut list = CircularSinglyLinkedList::new();
        assert!(list.is_empty());
        list.push_front(Foo(1));
        list.push_front(Foo(2));
        list.push_front(Foo(3));
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front().unwrap().0, 3);
        println!("after pop_fornt()");
        assert_eq!(list.len(), 2);
    }
    unsafe {
        let cnt = COUNT;
        assert_eq!(cnt, 3);
    }
    println!("leave");
}
