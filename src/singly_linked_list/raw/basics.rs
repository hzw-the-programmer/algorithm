use core::ptr;

pub(super) struct Node<T> {
    pub(super) value: T,
    pub(super) next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next: ptr::null_mut(),
        }
    }
}

pub struct List<T> {
    pub(super) head: *mut Node<T>,
    pub(super) tail: *mut Node<T>,
    pub(super) len: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, value: T) {
        let new_head = Box::into_raw(Box::new(Node::new(value)));
        if self.head.is_null() {
            self.tail = new_head;
        } else {
            unsafe {
                (*new_head).next = self.head;
            }
        }
        self.head = new_head;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            let old_head = unsafe { Box::from_raw(self.head) };
            self.head = old_head.next;
            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }
            self.len -= 1;
            Some(old_head.value)
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_tail = Box::into_raw(Box::new(Node::new(value)));
        if self.tail.is_null() {
            self.head = new_tail;
        } else {
            unsafe {
                (*self.tail).next = new_tail;
            }
        }
        self.tail = new_tail;
        self.len += 1;
    }

    pub fn clear(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> List<T> {
    pub fn push(&mut self, value: T) {
        self.push_back(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.peek_front()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.peek_front_mut()
    }

    pub fn peek_front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|n| &n.value) }
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|n| &mut n.value) }
    }
}

impl<T> List<T> {
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.len() {
            None
        } else {
            let mut cur = self.head;
            for _ in 0..idx {
                cur = unsafe { (*cur).next };
            }
            unsafe { cur.as_ref().map(|node| &node.value) }
        }
    }
}
