use super::*;

#[test]
fn test() {
    {
        let mut list = List::new();
        list.push_front(Foo(2));
        list.push_front(Foo(1));
        list.push_front(Foo(0));

        list.pop_front();
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
