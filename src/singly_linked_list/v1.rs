type BoxNode<T> = Box<Node<T>>;
type Link<T> = Option<BoxNode<T>>;
type LinkRef<'a, T> = Option<&'a BoxNode<T>>;

pub struct Node<T> {
    pub(crate) value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    // fn new(value: T) -> Self {
    //     Self { value, next: None }
    // }

    fn new_link(value: T, next: Link<T>) -> Link<T> {
        Some(Box::new(Node { value, next }))
    }
}

pub struct SinglyLinkedList<T> {
    head: Link<T>,
}

impl<T: std::fmt::Debug> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_back(&mut self, value: T) {
        match &mut self.head {
            None => self.head = Node::new_link(value, None),
            Some(node) => {
                let mut current = node;
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Node::new_link(value, None);
            }
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.head = Node::new_link(value, self.head.take());
    }

    pub fn middle_node(&self) -> LinkRef<T> {
        let mut slow = self.head.as_ref();
        let mut fast = self.head.as_ref();
        while fast.is_some() && fast.unwrap().next.is_some() {
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
            slow = slow.unwrap().next.as_ref();
        }
        slow
    }

    pub fn nth_from_end(&self, n: usize) -> LinkRef<T> {
        let mut fast = self.head.as_ref();
        for _ in 0..n {
            if fast.is_some() {
                fast = fast.unwrap().next.as_ref();
            } else {
                return None;
            }
        }

        let mut slow = self.head.as_ref();
        while fast.is_some() {
            fast = fast.unwrap().next.as_ref();
            slow = slow.unwrap().next.as_ref();
        }
        slow
    }

    pub fn remove_nth_from_end(&mut self, n: usize) -> Option<T> {
        assert!(n != 0);

        let mut fast = unsafe { &*(&self.head as *const Link<T>) }.as_ref();
        for _ in 0..n {
            if fast.is_some() {
                fast = fast.unwrap().next.as_ref();
            } else {
                return None;
            }
        }

        let mut dummy = Box::new(Node {
            value: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
            next: self.head.take(),
        });

        let mut slow = &mut dummy;
        while fast.is_some() {
            fast = fast.unwrap().next.as_ref();
            slow = slow.next.as_mut().unwrap();
        }

        let removed = slow.next.take().unwrap();
        slow.next = removed.next;
        self.head = dummy.next;
        let _ = std::mem::ManuallyDrop::new(dummy.value);
        Some(removed.value)
    }
}
