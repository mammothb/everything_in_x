use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let seen: HashSet<i32> = nums.into_iter().collect();
        let mut result = 0;
        for &num in &seen {
            if !seen.contains(&(num - 1)) {
                let mut count = 1;
                while seen.contains(&(num + count)) {
                    count += 1;
                }
                result = result.max(count);
            }
        }
        result
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2])
    );
}
