struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut result: Vec<Vec<i32>> = Vec::new();
        let n = nums.len();
        nums.sort_unstable();
        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..n {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let curr = (nums[i] + nums[j]) as i64;
                let mut l = j + 1;
                let mut r = n - 1;
                while l < r {
                    let total = curr + nums[l] as i64 + nums[r] as i64;
                    if total > target as i64 {
                        r -= 1;
                    } else if total < target as i64 {
                        l += 1;
                    } else {
                        result.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                        l += 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                        r -= 1;
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    }
                }
            }
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
