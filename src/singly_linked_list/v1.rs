type BoxNode<T> = Box<Node<T>>;
type Link<T> = Option<BoxNode<T>>;
type LinkRef<'a, T> = Option<&'a BoxNode<T>>;

pub struct Node<T> {
    pub(crate) value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }

    fn new_link(value: T) -> Link<T> {
        Some(Box::new(Node::new(value)))
    }
}

pub struct SinglyLinkedList<T> {
    head: Link<T>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_back(&mut self, value: T) {
        match &mut self.head {
            None => self.head = Node::new_link(value),
            Some(node) => {
                let mut current = node;
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Node::new_link(value);
            }
        }
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
}
