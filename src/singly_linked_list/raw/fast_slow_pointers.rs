use super::*;

impl<T> List<T> {
    pub fn middle(&self) -> Option<&T> {
        unsafe {
            let mut fast = self.head;
            let mut slow = self.head;
            while !fast.is_null() {
                fast = (*fast).next;
                if !fast.is_null() {
                    fast = (*fast).next;
                    slow = (*slow).next;
                }
            }

            slow.as_ref().map(|node| &node.value)
        }
    }
}
