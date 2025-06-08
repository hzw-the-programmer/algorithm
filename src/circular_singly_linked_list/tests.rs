use super::*;

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
        list.push_front(Foo(1));
        println!("leave block");
    }
    unsafe {
        let cnt = COUNT;
        assert_eq!(cnt, 1);
    }
    println!("leave");
}

#[test]
fn test_drop_2() {
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
        list.push_front(Foo(1));
        list.push_front(Foo(2));
        println!("leave block");
    }
    unsafe {
        let cnt = COUNT;
        assert_eq!(cnt, 2);
    }
    println!("leave");
}

#[test]
fn test_drop_3() {
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
        list.push_back(Foo(1));
        println!("leave block");
    }
    unsafe {
        let cnt = COUNT;
        assert_eq!(cnt, 1);
    }
    println!("leave");
}

#[test]
fn test_drop_4() {
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
        list.push_back(Foo(1));
        list.push_back(Foo(2));
        println!("leave block");
    }
    unsafe {
        let cnt = COUNT;
        assert_eq!(cnt, 2);
    }
    println!("leave");
}

// #[test]
// fn test_drop_1() {
//     static mut COUNT: i32 = 0;
//     struct Foo(i32);
//     impl Drop for Foo {
//         fn drop(&mut self) {
//             println!("Foo {} drop", self.0);
//             unsafe {
//                 COUNT += 1;
//             }
//         }
//     }

//     {
//         let mut list = CircularSinglyLinkedList::new();
//         assert!(list.is_empty());
//         list.push_front(Foo(1));
//         list.push_front(Foo(2));
//         list.push_front(Foo(3));
//         assert_eq!(list.len(), 3);
//         assert_eq!(list.pop_front().unwrap().0, 3);
//         println!("after pop_fornt()");
//         assert_eq!(list.len(), 2);
//     }
//     unsafe {
//         let cnt = COUNT;
//         assert_eq!(cnt, 3);
//     }
//     println!("leave");
// }

// #[test]
// fn test_drop_2() {
//     static mut COUNT: i32 = 0;
//     struct Foo(i32);
//     impl Drop for Foo {
//         fn drop(&mut self) {
//             println!("Foo {} drop", self.0);
//             unsafe {
//                 COUNT += 1;
//             }
//         }
//     }

//     {
//         let mut list = CircularSinglyLinkedList::new();
//         assert!(list.is_empty());

//         list.push_back(Foo(3));
//         list.push_front(Foo(2));
//         list.push_back(Foo(4));
//         println!("jhh");
//         list.push_front(Foo(1));
//         println!("asdfaf");
//         list.push_front(Foo(0));
//         println!("afa");
//         list.push_back(Foo(5));
//         assert_eq!(list.len(), 6);

//         assert_eq!(list.pop_front().unwrap().0, 0);
//         assert_eq!(list.pop_front().unwrap().0, 1);
//         assert_eq!(list.pop_front().unwrap().0, 2);
//         assert_eq!(list.pop_front().unwrap().0, 3);
//         assert_eq!(list.pop_front().unwrap().0, 4);
//         assert_eq!(list.pop_front().unwrap().0, 5);
//         assert_eq!(list.len(), 0);
//     }
//     // unsafe {
//     //     let cnt = COUNT;
//     //     assert_eq!(cnt, 6);
//     // }
//     println!("leave");
// }
