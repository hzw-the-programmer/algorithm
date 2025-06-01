use super::*;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct Iter<'a, T> {
    current: Tree<T>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Iter<'a, T> {
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
                    unsafe {
                        let ptr: *const T = &node.borrow().value;
                        return Some(&*ptr);
                    }
                }
            } else {
                self.current = node.borrow().right.clone();
                unsafe {
                    let ptr: *const T = &node.borrow().value;
                    return Some(&*ptr);
                }
            }
        }

        None
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.morris()
    }
}

impl<'a, T> Drop for Iter<'a, T> {
    fn drop(&mut self) {
        while let Some(_) = self.morris() {}
    }
}

impl<T> AVLTree<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.root.clone(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a AVLTree<T> {
    type IntoIter = Iter<'a, T>;
    type Item = <Iter<'a, T> as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
