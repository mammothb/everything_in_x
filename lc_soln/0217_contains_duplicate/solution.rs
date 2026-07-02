use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        for num in nums {
            if seen.contains(&num) {
                return true;
            }
            seen.insert(num);
        }
        false
    }
}

fn main() {
    println!("{}", Solution::has_duplicate(vec![1, 2, 3, 3]));
}
