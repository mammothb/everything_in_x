use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::<i32, usize>::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(&j) = seen.get(&num) {
                return vec![j as i32, i as i32];
            }
            seen.insert(target - num, i);
        }
        vec![]
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}
