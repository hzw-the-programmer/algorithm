use super::*;

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
