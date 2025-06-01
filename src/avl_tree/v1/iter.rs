use super::*;
use std::rc::Rc;

pub struct Iter<T> {
    current: Tree<T>,
}

impl<T> Iter<T> {
    fn morris(&mut self) -> Option<<Self as Iterator>::Item> {
        while let Some(node) = self.current.clone() {
            let left = node.borrow().left.clone();
            if let Some(left) = left {
                let mut rightmost = left.clone();
                while let Some(right) = {
                    let temp = rightmost.borrow().right.clone();
                    temp
                } {
                    if Rc::ptr_eq(&right, &node) {
                        break;
                    }
                    rightmost = right
                }
                if rightmost.borrow().right.is_none() {
                    rightmost.borrow_mut().right = Some(node);
                    self.current = Some(left);
                } else {
                    rightmost.borrow_mut().right = None;
                    self.current = node.borrow().right.clone();
                    return Some(node);
                }
            } else {
                self.current = node.borrow().right.clone();
                return Some(node);
            }
        }

        None
    }
}

impl<T> Iterator for Iter<T> {
    type Item = RcNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.morris()
    }
}

impl<T> Drop for Iter<T> {
    fn drop(&mut self) {
        while let Some(_) = self.morris() {}
    }
}

impl<T> AVLTree<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.root.clone(),
        }
    }
}

impl<T> IntoIterator for &AVLTree<T> {
    type IntoIter = Iter<T>;
    type Item = <Iter<T> as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
