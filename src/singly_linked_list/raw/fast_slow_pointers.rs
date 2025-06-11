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

    pub fn nth_from_end(&self, n: usize) -> Option<&T> {
        if n == 0 || n > self.len() {
            None
        } else {
            unsafe {
                let mut fast = self.head;
                for _ in 0..n {
                    fast = (*fast).next;
                }
                let mut slow = self.head;
                while !fast.is_null() {
                    fast = (*fast).next;
                    slow = (*slow).next;
                }
                slow.as_ref().map(|node| &node.value)
            }
        }
    }
}
