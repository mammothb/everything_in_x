struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut result = nums.len() as i32 + 1;
        let mut curr = 0;
        let mut start = 0;
        for (i, num) in nums.iter().enumerate() {
            curr += num;
            while curr >= target {
                result = result.min((i - start + 1) as i32);
                curr -= nums[start];
                start += 1;
            }
        }
        if result == nums.len() as i32 + 1 {
            0
        } else {
            result
        }
    }
}

fn main() {
    println!("Hello, world!");
}
