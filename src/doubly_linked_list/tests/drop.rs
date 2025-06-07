use super::*;

#[test]
fn test() {
    {
        let mut dll = DoublyLinkedList::new();
        dll.push_front(Foo(2));
        dll.push_front(Foo(1));
        dll.push_front(Foo(0));

        dll.pop_front();
        unsafe {
            let count = COUNT;
            assert_eq!(count, 1);
        }
        println!("leave block");
    }
    unsafe {
        let count = COUNT;
        assert_eq!(count, 3);
    }
    println!("leave test");
}

static mut COUNT: i32 = 0;

struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        unsafe {
            COUNT += 1;
        }
        println!("Foo {} drop", self.0);
    }
}
