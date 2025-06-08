use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Rc<RefCell<Node<T>>>>) -> Self {
        Self { value, next }
    }
}

pub struct CircularSinglyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> CircularSinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node::new(value, None)));
        if self.head.is_none() {
            node.borrow_mut().next = Some(node.clone());
            self.tail = Some(Rc::downgrade(&node));
        } else {
            node.borrow_mut().next = self.head.take();
            let tail = self.tail.as_ref().unwrap().upgrade().unwrap();
            tail.borrow_mut().next = Some(node.clone());
        }
        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let node = self.head.take().unwrap();
        if let Some(next) = node.borrow_mut().next.take() {
            if Rc::ptr_eq(&node, &next) {
                self.tail = None;
            } else {
                self.head = Some(next.clone());
                let tail = self.tail.as_ref().unwrap().upgrade().unwrap();
                tail.borrow_mut().next = Some(next);
            }
        }

        self.len -= 1;

        Some(Rc::into_inner(node).unwrap().into_inner().value)
    }

    pub fn push_back(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node::new(value, None)));
        let weak = Rc::downgrade(&node);
        if self.tail.is_none() {
            node.borrow_mut().next = Some(node.clone());
            self.head = Some(node);
        } else {
            let tail = self.tail.as_ref().unwrap().upgrade().unwrap();
            node.borrow_mut().next = tail.borrow_mut().next.take();
            tail.borrow_mut().next = Some(node);
        }
        self.tail = Some(weak);
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
        if let Some(tail) = self.tail.as_ref() {
            let tail = tail.upgrade().unwrap();
            tail.borrow_mut().next.take();
        }
    }
}
