pub struct SegmentTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegmentTree {
    pub fn new(arr: &[i32]) -> Self {
        let n = arr.len();

        let mut tree = vec![0; 4 * n];
        if n > 0 {
            Self::build(&mut tree, 0, 0, n - 1, arr);
        }

        SegmentTree { tree, n }
    }

    fn build(tree: &mut Vec<i32>, node: usize, start: usize, end: usize, arr: &[i32]) {
        if start == end {
            tree[node] = arr[start];
        } else {
            let mid = (start + end) / 2;
            let left = 2 * node + 1;
            let right = 2 * node + 2;

            Self::build(tree, left, start, mid, arr);
            Self::build(tree, right, mid + 1, end, arr);

            tree[node] = tree[left] + tree[right];
        }
    }

    pub fn query(&self, l: usize, r: usize) -> i32 {
        if l <= r && l < self.n {
            self.query_helper(0, 0, self.n - 1, l, r)
        } else {
            0
        }
    }

    fn query_helper(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i32 {
        if r < start || l > end {
            0
        } else if l <= start && r >= end {
            self.tree[node]
        } else {
            let mid = (start + end) / 2;
            let left = 2 * node + 1;
            let right = 2 * node + 2;
            let left_sum = self.query_helper(left, start, mid, l, r);
            let right_sum = self.query_helper(right, mid + 1, end, l, r);
            left_sum + right_sum
        }
    }

    pub fn update(&mut self, index: usize, value: i32) {
        if index < self.n {
            self.update_helper(0, 0, self.n - 1, index, value);
        }
    }

    fn update_helper(&mut self, node: usize, start: usize, end: usize, index: usize, value: i32) {
        if start == end {
            self.tree[node] = value;
        } else {
            let mid = (start + end) / 2;
            let left = 2 * node + 1;
            let right = 2 * node + 2;
            if index <= mid {
                self.update_helper(left, start, mid, index, value);
            } else {
                self.update_helper(right, mid + 1, end, index, value);
            }
            self.tree[node] = self.tree[left] + self.tree[right];
        }
    }
}
