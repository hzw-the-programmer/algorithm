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
        self.nth_node_from_end(n).map(|node| &node.value)
    }

    fn nth_node_from_end(&self, n: usize) -> Option<&mut Node<T>> {
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
                slow.as_mut()
            }
        }
    }

    pub fn remove_nth_from_end(&mut self, n: usize) -> Option<T> {
        if n > 0 && n < self.len() {
            let pre = self.nth_node_from_end(n + 1).unwrap();
            let cur = pre.next;
            pre.next = unsafe { (*cur).next };
            if n == 1 {
                self.tail = pre;
            }
            let node = unsafe { Box::from_raw(cur) };
            self.len -= 1;
            Some(node.value)
        } else if n == self.len() {
            self.pop_front()
        } else {
            None
        }
    }
}
