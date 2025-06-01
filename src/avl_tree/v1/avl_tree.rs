use std::cell::RefCell;
use std::rc::Rc;

pub(super) type RcNode<T> = Rc<RefCell<Node<T>>>;
pub type Tree<T> = Option<RcNode<T>>;

#[derive(Debug)]
pub struct Node<T> {
    pub(crate) value: T,
    pub(crate) left: Tree<T>,
    pub(crate) right: Tree<T>,
    height: i32,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
            height: 1,
        }
    }

    pub(super) fn new_tree(value: T) -> Tree<T> {
        Some(Rc::new(RefCell::new(Node::new(value))))
    }

    fn height(&self) -> i32 {
        self.height
    }

    pub(super) fn update_height(&mut self) {
        let left = self
            .left
            .as_deref()
            .map(|n| n.borrow().height())
            .unwrap_or(0);
        let right = self
            .right
            .as_deref()
            .map(|n| n.borrow().height())
            .unwrap_or(0);
        self.height = 1 + left.max(right)
    }

    pub(super) fn balance(&self) -> i32 {
        let left = self
            .left
            .as_deref()
            .map(|n| n.borrow().height())
            .unwrap_or(0);
        let right = self
            .right
            .as_deref()
            .map(|n| n.borrow().height())
            .unwrap_or(0);
        left - right
    }

    /*
            y                               x
           / \     Right Rotation          /  \
          x   T3   - - - - - - - >        T1   y
         / \       < - - - - - - -            / \
        T1  T2     Left Rotation            T2  T3
    */
    pub(super) fn right_rotate(y: RcNode<T>) -> RcNode<T> {
        let x = y.borrow_mut().left.take().unwrap();
        let t2 = x.borrow_mut().right.take();

        y.borrow_mut().left = t2;
        x.borrow_mut().right = Some(y.clone());

        y.borrow_mut().update_height();
        x.borrow_mut().update_height();

        x
    }

    pub(super) fn left_rotate(x: RcNode<T>) -> RcNode<T> {
        let y = x.borrow_mut().right.take().unwrap();
        let t2 = y.borrow_mut().left.take();

        x.borrow_mut().right = t2;
        y.borrow_mut().left = Some(x.clone());

        x.borrow_mut().update_height();
        y.borrow_mut().update_height();

        y
    }
}

pub struct AVLTree<T> {
    pub(crate) root: Tree<T>,
}

impl<T> AVLTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}
