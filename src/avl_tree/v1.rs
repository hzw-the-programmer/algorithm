use std::cell::RefCell;
use std::cmp::PartialOrd;
use std::rc::Rc;

type RcNode<T> = Rc<RefCell<Node<T>>>;
type Tree<T> = Option<RcNode<T>>;

struct Node<T> {
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
}

struct AVLTree<T> {
    root: Tree<T>,
}

impl<T: PartialOrd + Clone> AVLTree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: T) {
        self.root = Self::insert_recursive(self.root.take(), &value);
    }

    fn insert_recursive(tree: Tree<T>, value: &T) -> Tree<T> {
        match tree {
            None => Some(Rc::new(RefCell::new(Node::new(value.clone())))),
            Some(node) => {
                if *value < node.borrow().value {
                    node.borrow_mut().left =
                        Self::insert_recursive(node.borrow_mut().left.take(), value);
                } else if *value > node.borrow().value {
                    node.borrow_mut().right =
                        Self::insert_recursive(node.borrow_mut().right.take(), value);
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
