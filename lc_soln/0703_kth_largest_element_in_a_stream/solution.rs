use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: i32,
    q: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut q = BinaryHeap::new();
        for num in nums {
            q.push(Reverse(num));
        }
        while q.len() > k as usize {
            q.pop();
        }
        Self { k, q }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.q.push(Reverse(val));
        if self.q.len() > self.k as usize {
            self.q.pop();
        }
        self.q.peek().unwrap().0
    }
}

fn main() {}
