use std::cell::RefCell;
use std::rc::Rc;

pub type Tree<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(PartialEq, Debug)]
pub struct Node<T> {
    pub(crate) value: T,
    pub(crate) left: Tree<T>,
    pub(crate) right: Tree<T>,
}

pub fn new_tree<T>(value: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
    Some(Rc::new(RefCell::new(Node { value, left, right })))
}

pub fn new_tree_from<T: Clone>(arr: &[Option<T>]) -> Tree<T> {
    new_tree_recursive(arr, 0)
}

fn new_tree_recursive<T: Clone>(arr: &[Option<T>], index: usize) -> Tree<T> {
    if index < arr.len() {
        if let Some(value) = &arr[index] {
            return new_tree(
                value.clone(),
                new_tree_recursive(arr, 2 * index + 1),
                new_tree_recursive(arr, 2 * index + 2),
            );
        }
    }
    None
}
