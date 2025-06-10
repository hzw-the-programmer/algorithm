use std::ptr;

pub type BoxNode<T> = Box<Node<T>>;
pub type Link<T> = Option<BoxNode<T>>;
pub type LinkRef<'a, T> = Option<&'a BoxNode<T>>;

pub struct Node<T> {
    pub(crate) value: T,
    pub(crate) next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T, next: Link<T>) -> Self {
        Self { value, next }
    }

    pub fn new_box(value: T, next: Link<T>) -> BoxNode<T> {
        Box::new(Node::new(value, next))
    }

    pub fn new_link(value: T, next: Link<T>) -> Link<T> {
        Some(Box::new(Node { value, next }))
    }
}

pub type SinglyLinkedList<T> = List<T>;

pub struct List<T> {
    pub(crate) head: Link<T>,
    pub(super) tail: *mut Node<T>,
    pub(super) len: usize,
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
        let mut node = Node::new_box(value, self.head.take());
        if self.tail.is_null() {
            self.tail = &mut *node as *mut Node<T>;
        }
        self.head = Some(node);
        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let mut new_tail = Box::new(Node::new(value, None));
        let raw_tail: *mut _ = &mut *new_tail;
        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            unsafe { (*self.tail).next = Some(new_tail) };
        }
        self.tail = raw_tail;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            self.len -= 1;
            old_head.value
        })
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = ptr::null_mut();
        self.len = 0;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> List<T> {
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.value)
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_deref_mut().map(|node| &mut node.value)
    }

    pub fn push(&mut self, value: T) {
        self.push_back(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }
}
