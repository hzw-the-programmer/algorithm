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
            let node = unsafe { Box::from_raw(pre.next) };
            pre.next = node.next;
            if n == 1 {
                self.tail = pre;
            }
            self.len -= 1;
            Some(node.value)
        } else if n == self.len() {
            self.pop_front()
        } else {
            None
        }
    }

    pub fn has_cycle(&self) -> bool {
        let mut fast = self.head;
        let mut slow = self.head;
        while !fast.is_null() {
            fast = unsafe { (*fast).next };
            if !fast.is_null() {
                fast = unsafe { (*fast).next };
                slow = unsafe { (*slow).next };
                if fast == slow {
                    return true;
                }
            }
        }
        false
    }

    pub fn cycle_entry(&self) -> Option<&T> {
        let mut fast = self.head;
        let mut slow = self.head;
        while !fast.is_null() {
            fast = unsafe { (*fast).next };
            if !fast.is_null() {
                fast = unsafe { (*fast).next };
                slow = unsafe { (*slow).next };
                if fast == slow {
                    fast = self.head;
                    break;
                }
            }
        }

        while !fast.is_null() {
            fast = unsafe { (*fast).next };
            slow = unsafe { (*slow).next };
            if fast == slow {
                return unsafe { slow.as_ref().map(|node| &node.value) };
            }
        }

        None
    }
}

#[cfg(test)]
impl<T> List<T> {
    pub fn link(&mut self, from: usize, to: usize) {
        if from >= self.len() || to >= self.len() {
            return;
        }

        let to: *mut _ = self.get_mut_node(to).unwrap();
        let from = self.get_mut_node(from).unwrap();
        from.next = to;
    }

    pub fn unlink(&mut self, from: usize, to: usize) {
        if from >= self.len() || to >= self.len() {
            return;
        }

        let to: *mut _ = self.get_mut_node(to).unwrap();
        let from = self.get_mut_node(from).unwrap();
        if core::ptr::eq(from.next, to) {
            from.next = core::ptr::null_mut();
        }
    }
}
