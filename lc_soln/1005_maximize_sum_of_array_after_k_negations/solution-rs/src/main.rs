use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut total = nums.iter().sum();
        let mut max_heap: BinaryHeap<Reverse<i32>> =
            BinaryHeap::from(nums.into_iter().map(Reverse).collect::<Vec<Reverse<i32>>>());
        let mut k = k;
        while k > 0
            && let Some(mut num) = max_heap.peek_mut()
        {
            if num.0 == 0 {
                break;
            }
            total -= 2 * num.0;
            *num = Reverse(-num.0);
            k -= 1;
        }
        total
    }
}

fn main() {
    println!(
        "{}",
        Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1)
    );
}
