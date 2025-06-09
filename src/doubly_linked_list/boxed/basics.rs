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

pub struct List<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut new_head = Box::new(Node::new(value, None, ptr::null_mut()));
        match self.head.take() {
            Some(mut old_head) => {
                old_head.pre = &mut *new_head;
                new_head.next = Some(old_head);
                self.head = Some(new_head);
                self.len += 1;
            }
            None => {
                self.tail = &mut *new_head;
                self.head = Some(new_head);
                self.len += 1;
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.len -= 1;
            match old_head.next {
                Some(mut new_head) => {
                    new_head.pre = ptr::null_mut();
                    self.head = Some(new_head);
                }
                None => self.tail = ptr::null_mut(),
            }
            old_head.value
        })
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

        self.tail = unsafe { (*self.tail).pre };
        // let mut node = if self.tail.is_null() {
        let node = if self.tail.is_null() {
            self.head.take().unwrap()
        } else {
            unsafe { (*self.tail).next.take().unwrap() }
        };
        // node.pre = ptr::null_mut();
        self.len -= 1;

        Some(node.value)
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

impl<T> List<T> {
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_deref().map(|head| &head.value)
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_deref_mut().map(|head| &mut head.value)
    }
}
