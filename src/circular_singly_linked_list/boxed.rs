use core::mem::ManuallyDrop;
use core::ptr;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

pub struct CircularSinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> CircularSinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut node = Box::new(Node::new(value));
        if self.head.is_none() {
            node.next = Some(unsafe { Box::from_raw(&mut *node) });
            self.tail = &mut *node;
        } else {
            node.next = self.head.take();
            unsafe {
                let _ = ManuallyDrop::new((*self.tail).next.take());
                (*self.tail).next = Some(Box::from_raw(&mut *node));
            }
        }
        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let mut node = self.head.take().unwrap();
        if ptr::eq(self.tail, &*node) {
            let _ = ManuallyDrop::new(node.next.take());
            self.tail = ptr::null_mut();
        } else {
            let mut next = node.next.take().unwrap();
            unsafe {
                let _ = ManuallyDrop::new((*self.tail).next.take());
                (*self.tail).next = Some(Box::from_raw(&mut *next));
            }
            self.head = Some(next);
        }
        self.len -= 1;

        Some(node.value)
    }

    pub fn push_back(&mut self, value: T) {
        let mut node = Box::new(Node::new(value));
        let raw = &mut *node as *mut Node<T>;
        if self.tail.is_null() {
            node.next = Some(unsafe { Box::from_raw(raw) });
            self.head = Some(node);
        } else {
            unsafe {
                node.next = (*self.tail).next.take();
                (*self.tail).next = Some(node);
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
}

impl<T> Drop for CircularSinglyLinkedList<T> {
    fn drop(&mut self) {
        if !self.tail.is_null() {
            let _ = ManuallyDrop::new(unsafe { (*self.tail).next.take() });
        }
    }
}
