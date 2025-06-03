use std::ptr::NonNull;

type BoxNode<T> = Box<Node<T>>;
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

    fn new_box(value: T, next: Link<T>) -> BoxNode<T> {
        Box::new(Node::new(value, next))
    }

    // fn new_link(value: T, next: Link<T>) -> Link<T> {
    //     Some(Box::new(Node { value, next }))
    // }
}

pub struct SinglyLinkedList<T> {
    pub(crate) head: Link<T>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut node = Node::new_box(value, self.head.take());
        if self.tail.is_none() {
            self.tail = unsafe { Some(NonNull::new_unchecked(&mut *node as *mut Node<T>)) };
        }
        self.head = Some(node);
        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let mut node = Node::new_box(value, None);
        let ptr = &mut *node as *mut Node<T>;
        match self.tail {
            None => self.head = Some(node),
            Some(mut ptr) => unsafe { ptr.as_mut().next = Some(node) },
        }
        self.tail = unsafe { Some(NonNull::new_unchecked(ptr)) };
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            if self.head.is_none() {
                self.tail = None;
            }
            self.len -= 1;
            node.value
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
