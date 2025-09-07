use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut curr = 0;
        let mut result = 0;
        let mut start = 0;
        for num in &nums {
            while seen.contains(num) {
                seen.remove(&nums[start]);
                curr -= nums[start];
                start += 1;
            }
            seen.insert(*num);
            curr += num;
            result = result.max(curr);
        }
        result
    }
}

fn main() {
    println!("{}", Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]));
}
