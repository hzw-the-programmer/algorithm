use core::ptr;

struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next: ptr::null_mut(),
        }
    }
}

pub struct CircularSinglyLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> CircularSinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let raw = Box::into_raw(Box::new(Node::new(value)));
        if self.head.is_null() {
            unsafe {
                (*raw).next = raw;
            }
            self.tail = raw;
        } else {
            unsafe {
                (*raw).next = self.head;
                (*self.tail).next = raw;
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
        if ptr::eq(node.next, &*node) {
            self.head = ptr::null_mut();
            self.tail = ptr::null_mut();
        } else {
            self.head = node.next;
            unsafe {
                (*self.tail).next = node.next;
            }
        }
        // node.next = ptr::null_mut();
        self.len -= 1;

        Some(node.value)
    }

    pub fn push_back(&mut self, value: T) {
        let raw = Box::into_raw(Box::new(Node::new(value)));
        if self.tail.is_null() {
            unsafe {
                (*raw).next = raw;
            }
            self.head = raw;
        } else {
            unsafe {
                (*raw).next = self.head;
                (*self.tail).next = raw;
            }
        }
        self.tail = raw;
        self.len += 1;
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> Drop for CircularSinglyLinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}
