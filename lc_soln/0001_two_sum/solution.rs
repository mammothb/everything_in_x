use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();
        let mut result = Vec::new();
        for (i, num) in nums.into_iter().enumerate() {
            if let Some(&j) = seen.get(&(target - num)) {
                result.push(i as i32);
                result.push(j as i32);
                break;
            }
            seen.insert(num, i);
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}
