struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            if num > 0 {
                break;
            }
            if i > 0 && num == nums[i - 1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                match num + nums[l] + nums[r] {
                    total if total < 0 => l += 1,
                    total if total > 0 => r -= 1,
                    _ => {
                        result.push(vec![num, nums[l], nums[r]]);
                        r -= 1;
                        l += 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1
                        }
                    }
                }
            }
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
}
