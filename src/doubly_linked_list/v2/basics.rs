use core::ptr;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
    pre: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>, pre: *mut Node<T>) -> Self {
        Self { value, next, pre }
    }
}

pub struct DoublyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut node = Box::new(Node::new(value, self.head.take(), ptr::null_mut()));
        if node.next.is_none() {
            self.tail = &mut *node;
        } else {
            node.next.as_deref_mut().unwrap().pre = &mut *node;
        }
        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let mut node = self.head.take().unwrap();
        self.head = node.next.take();
        if self.head.is_none() {
            self.tail = ptr::null_mut();
        } else {
            self.head.as_deref_mut().unwrap().pre = ptr::null_mut();
        }
        self.len -= 1;

        Some(node.value)
    }

    pub fn push_back(&mut self, value: T) {
        let mut node = Box::new(Node::new(value, None, self.tail));
        let raw = &mut *node as *mut Node<T>;
        if self.tail.is_null() {
            self.head = Some(node);
        } else {
            unsafe {
                (*self.tail).next = Some(node);
            }
        }
        self.tail = raw;
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }

        self.len -= 1;

        let pre = unsafe { (*self.tail).pre };
        if pre.is_null() {
            self.tail = ptr::null_mut();
            Some(self.head.take().unwrap().value)
        } else {
            self.tail = pre;
            unsafe { Some((*self.tail).next.take().unwrap().value) }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
