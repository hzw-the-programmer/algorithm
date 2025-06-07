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

        let node = unsafe { Box::from_raw(self.head) };
        self.head = node.next;
        if self.head.is_null() {
            self.tail = ptr::null_mut();
        } else {
            unsafe {
                (*node.next).pre = ptr::null_mut();
            }
        }
        self.len -= 1;

        // node.next = ptr::null_mut();
        Some(node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
