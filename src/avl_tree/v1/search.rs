use super::*;

impl<T: PartialOrd> AVLTree<T> {
    pub fn search(&self, value: &T) -> bool {
        Self::search_recursive(&self.root, value)
    }

    fn search_recursive(root: &Tree<T>, value: &T) -> bool {
        match root {
            None => false,
            Some(node) => {
                let borrowed = node.borrow();
                if *value < borrowed.value {
                    Self::search_recursive(&borrowed.left, value)
                } else if *value > borrowed.value {
                    Self::search_recursive(&borrowed.right, value)
                } else {
                    true
                }
            }
        }
    }
}
