use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut h = BinaryHeap::from_iter(stones);
        while h.len() > 1 {
            let x = h.pop().unwrap();
            let y = h.pop().unwrap();
            if x != y {
                h.push(x - y);
            }
        }
        if h.is_empty() {
            0
        } else {
            *h.peek().unwrap()
        }
    }
}

fn main() {}
