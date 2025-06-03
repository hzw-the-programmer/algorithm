use std::ptr;

type BoxNode<T> = Box<Node<T>>;
pub type Link<T> = Option<BoxNode<T>>;
pub type LinkRef<'a, T> = Option<&'a BoxNode<T>>;

pub struct Node<T> {
    pub(crate) value: T,
    pub(super) next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T, next: Link<T>) -> Self {
        Self { value, next }
    }

    fn new_box(value: T, next: Link<T>) -> BoxNode<T> {
        Box::new(Node::new(value, next))
    }

    // fn new_link(value: T, next: Link<T>) -> Link<T> {
    //     Some(Box::new(Node { value, next }))
    // }
}

pub struct SinglyLinkedList<T> {
    pub(super) head: Link<T>,
    pub(super) tail: *mut Node<T>,
    pub(super) len: usize,
}

impl<T> SinglyLinkedList<T> {
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
        let mut node = Node::new_box(value, None);
        let ptr = &mut *node as *mut Node<T>;
        if self.tail.is_null() {
            self.head = Some(node);
        } else {
            unsafe { (*self.tail).next = Some(node) };
        }
        self.tail = ptr;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            self.len -= 1;
            node.value
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
