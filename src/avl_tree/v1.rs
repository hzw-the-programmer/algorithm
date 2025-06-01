use std::cell::RefCell;
use std::cmp::PartialOrd;
use std::rc::Rc;

type RcNode<T> = Rc<RefCell<Node<T>>>;
pub type Tree<T> = Option<RcNode<T>>;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left: Tree<T>,
    right: Tree<T>,
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

    fn height(&self) -> i32 {
        self.height
    }

    fn update_height(&mut self) {
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

    fn balance(&self) -> i32 {
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
    fn right_rotate(y: RcNode<T>) -> RcNode<T> {
        let x = y.borrow_mut().left.take().unwrap();
        let t2 = x.borrow_mut().right.take();

        y.borrow_mut().left = t2;
        x.borrow_mut().right = Some(y.clone());

        y.borrow_mut().update_height();
        x.borrow_mut().update_height();

        x
    }

    fn left_rotate(x: RcNode<T>) -> RcNode<T> {
        let y = x.borrow_mut().right.take().unwrap();
        let t2 = y.borrow_mut().left.take();

        x.borrow_mut().right = t2;
        y.borrow_mut().left = Some(x.clone());

        x.borrow_mut().update_height();
        y.borrow_mut().update_height();

        y
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn left(&self) -> &Tree<T> {
        &self.left
    }

    pub fn right(&self) -> &Tree<T> {
        &self.right
    }
}

pub struct AVLTree<T> {
    root: Tree<T>,
}

impl<T> AVLTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn root(&self) -> &Tree<T> {
        &self.root
    }
}

impl<T: PartialOrd + Clone> AVLTree<T> {
    pub fn insert(&mut self, value: T) {
        self.root = Self::insert_recursive(self.root.take(), &value);
    }

    fn insert_recursive(tree: Tree<T>, value: &T) -> Tree<T> {
        match tree {
            None => Some(Rc::new(RefCell::new(Node::new(value.clone())))),
            Some(node) => {
                if *value < node.borrow().value {
                    let left = node.borrow_mut().left.take();
                    node.borrow_mut().left = Self::insert_recursive(left, value);
                } else if *value > node.borrow().value {
                    let right = node.borrow_mut().right.take();
                    node.borrow_mut().right = Self::insert_recursive(right, value);
                } else {
                    return Some(node);
                }

                node.borrow_mut().update_height();

                let balance = node.borrow().balance();

                if balance > 1 {
                    if *value < node.borrow().left.as_ref().unwrap().borrow().value {
                        // LL
                        Some(Node::right_rotate(node))
                    } else {
                        // LR
                        let left = node.borrow_mut().left.take().unwrap();
                        node.borrow_mut().left = Some(Node::left_rotate(left));
                        Some(Node::right_rotate(node))
                    }
                } else if balance < -1 {
                    if *value > node.borrow().right.as_ref().unwrap().borrow().value {
                        // RR
                        Some(Node::left_rotate(node))
                    } else {
                        // RL
                        let right = node.borrow_mut().right.take().unwrap();
                        node.borrow_mut().right = Some(Node::right_rotate(right));
                        Some(Node::left_rotate(node))
                    }
                } else {
                    Some(node)
                }
            }
        }
    }
}

impl<T: PartialOrd> AVLTree<T> {
    pub fn delete(&mut self, value: &T) {
        self.root = AVLTree::delete_recursive(self.root.take(), value)
    }

    fn delete_recursive(root: Tree<T>, value: &T) -> Tree<T> {
        match root {
            None => None,
            Some(node) => {
                let balance = {
                    let mut borrowed = node.borrow_mut();
                    if *value < borrowed.value {
                        borrowed.left = Self::delete_recursive(borrowed.left.take(), value);
                    } else if *value > borrowed.value {
                        borrowed.right = Self::delete_recursive(borrowed.right.take(), value);
                    } else {
                        if borrowed.left.is_none() {
                            return borrowed.right.take();
                        } else if borrowed.right.is_none() {
                            return borrowed.left.take();
                        } else {
                            let min_node = Self::min_node(borrowed.right.clone().unwrap());
                            std::mem::swap(&mut borrowed.value, &mut min_node.borrow_mut().value);
                            borrowed.right = Self::delete_recursive(borrowed.right.take(), value);
                        }
                    }

                    borrowed.update_height();
                    borrowed.balance()
                };

                if balance > 1 {
                    {
                        let mut borrowed = node.borrow_mut();
                        if borrowed.left.as_ref().unwrap().borrow().balance() < 0 {
                            let left = borrowed.left.take().unwrap();
                            borrowed.left = Some(Node::left_rotate(left));
                        }
                    }
                    Some(Node::right_rotate(node))
                } else if balance < -1 {
                    {
                        let mut borrowed = node.borrow_mut();
                        if borrowed.right.as_ref().unwrap().borrow().balance() > 0 {
                            let right = borrowed.right.take().unwrap();
                            borrowed.right = Some(Node::right_rotate(right));
                        }
                    }
                    Some(Node::left_rotate(node))
                } else {
                    Some(node)
                }
            }
        }
    }

    fn min_node(node: RcNode<T>) -> RcNode<T> {
        let mut leftmost = node;
        while let Some(left) = {
            let temp = leftmost.borrow().left.clone();
            temp
        } {
            leftmost = left;
        }
        leftmost
    }
}
