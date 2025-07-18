use super::*;

#[test]
fn test() {
    {
        let mut list = List::new();
        list.push_back(Foo(3));
        list.push_front(Foo(2));
        list.push_back(Foo(4));
        list.push_front(Foo(1));
        list.push_back(Foo(5));
        list.push_front(Foo(0));

        list.pop_front();
        unsafe {
            // assert_eq!(COUNT, 1);
            let count = COUNT;
            assert_eq!(count, 1);
        }

        list.pop_front();
        unsafe {
            assert!(COUNT == 2);
        }

        list.pop_front();
        unsafe {
            assert!(COUNT == 3);
        }

        println!("leave block");
    }
    unsafe {
        assert!(COUNT == 6);
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
