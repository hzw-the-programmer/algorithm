use core::ptr;

struct Node<T> {
    value: T,
    next: *mut Node<T>,
    pre: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(value: T, next: *mut Node<T>, pre: *mut Node<T>) -> Self {
        Self { value, next, pre }
    }
}

pub struct DoublyLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let raw = Box::into_raw(Box::new(Node::new(value, self.head, ptr::null_mut())));
        if self.head.is_null() {
            self.tail = raw;
        } else {
            unsafe {
                (*self.head).pre = raw;
            }
        }
        self.head = raw;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        // let mut node = unsafe { Box::from_raw(self.head) };
        let node = unsafe { Box::from_raw(self.head) };
        self.head = node.next;
        // node.next = ptr::null_mut();
        if self.head.is_null() {
            self.tail = ptr::null_mut();
        } else {
            unsafe {
                (*self.head).pre = ptr::null_mut();
            }
        }
        self.len -= 1;

        Some(node.value)
    }

    pub fn push_back(&mut self, value: T) {
        let raw = Box::into_raw(Box::new(Node::new(value, ptr::null_mut(), self.tail)));
        if self.tail.is_null() {
            self.head = raw;
        } else {
            unsafe {
                (*self.tail).next = raw;
            }
        }
        self.tail = raw;
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }

        // let mut node = unsafe { Box::from_raw(self.tail) };
        let node = unsafe { Box::from_raw(self.tail) };
        self.tail = node.pre;
        // node.pre = ptr::null_mut();
        if self.tail.is_null() {
            self.head = ptr::null_mut();
        } else {
            unsafe {
                (*self.tail).next = ptr::null_mut();
            }
        }
        self.len -= 1;

        Some(node.value)
    }

    pub fn clear(&mut self) {
        while let Some(_) = self.pop_front() {}
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}
