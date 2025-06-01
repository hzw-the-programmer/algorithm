use super::*;

impl<T: PartialOrd + Clone> AVLTree<T> {
    pub fn insert(&mut self, value: T) {
        self.root = Self::insert_recursive(self.root.take(), &value);
    }

    fn insert_recursive(tree: Tree<T>, value: &T) -> Tree<T> {
        match tree {
            None => Node::new_tree(value.clone()),
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
