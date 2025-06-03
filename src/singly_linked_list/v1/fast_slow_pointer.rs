use super::*;
use std::ptr;

impl<T: std::fmt::Debug> SinglyLinkedList<T> {
    pub fn middle(&self) -> LinkRef<T> {
        let mut slow = self.head.as_ref();
        let mut fast = self.head.as_ref();
        while fast.is_some() && fast.unwrap().next.is_some() {
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
            slow = slow.unwrap().next.as_ref();
        }
        slow
    }

    pub fn nth_from_end(&self, n: usize) -> LinkRef<T> {
        let mut fast = self.head.as_ref();
        for _ in 0..n {
            if fast.is_some() {
                fast = fast.unwrap().next.as_ref();
            } else {
                return None;
            }
        }

        let mut slow = self.head.as_ref();
        while fast.is_some() {
            fast = fast.unwrap().next.as_ref();
            slow = slow.unwrap().next.as_ref();
        }
        slow
    }

    pub fn remove_nth_from_end(&mut self, n: usize) -> Option<T> {
        assert!(n != 0);

        let mut fast = unsafe { &*(&self.head as *const Link<T>) }.as_ref();
        for _ in 0..n {
            if fast.is_some() {
                fast = fast.unwrap().next.as_ref();
            } else {
                return None;
            }
        }

        let mut dummy = Box::new(Node {
            value: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
            next: self.head.take(),
        });

        let mut slow = &mut dummy;
        while fast.is_some() {
            fast = fast.unwrap().next.as_ref();
            slow = slow.next.as_mut().unwrap();
        }

        let removed = slow.next.take().unwrap();
        slow.next = removed.next;

        self.head = dummy.next;
        if self.head.is_none() {
            self.tail = ptr::null_mut();
        }
        self.len -= 1;

        let _ = std::mem::ManuallyDrop::new(dummy.value);
        Some(removed.value)
    }

    pub fn has_cycle(&self) -> bool {
        let mut fast = self.head.as_deref();
        let mut slow = self.head.as_deref();
        while fast.is_some() {
            fast = fast.unwrap().next.as_deref();
            println!("fast: {:?}, {:p}", fast.unwrap().value, fast.unwrap());
            if fast.is_some() {
                fast = fast.unwrap().next.as_deref();
                println!("fast: {:?}, {:p}", fast.unwrap().value, fast.unwrap());
                slow = slow.unwrap().next.as_deref();
                println!("slow: {:?}, {:p}", slow.unwrap().value, slow.unwrap());
                if fast.is_some()
                    && fast.unwrap() as *const Node<T> == slow.unwrap() as *const Node<T>
                {
                    println!("found");
                    return true;
                }
            }
        }
        false
    }

    pub fn cycle_entry(&self) -> Option<&Node<T>> {
        let mut fast = self.head.as_deref();
        let mut slow = self.head.as_deref();
        while fast.is_some() {
            fast = fast.unwrap().next.as_deref();
            println!("fast: {:?}, {:p}", fast.unwrap().value, fast.unwrap());
            if fast.is_some() {
                fast = fast.unwrap().next.as_deref();
                println!("fast: {:?}, {:p}", fast.unwrap().value, fast.unwrap());
                slow = slow.unwrap().next.as_deref();
                println!("slow: {:?}, {:p}", slow.unwrap().value, slow.unwrap());
                if fast.is_some()
                    && fast.unwrap() as *const Node<T> == slow.unwrap() as *const Node<T>
                {
                    println!("found");
                    break;
                }
            }
        }

        if fast.is_none() {
            return None;
        }

        fast = self.head.as_deref();
        while fast.unwrap() as *const Node<T> != slow.unwrap() as *const Node<T> {
            fast = fast.unwrap().next.as_deref();
            slow = slow.unwrap().next.as_deref();
        }
        // None
        Some(slow.unwrap())
    }
}
