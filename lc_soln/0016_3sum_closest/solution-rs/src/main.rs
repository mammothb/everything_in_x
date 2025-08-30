struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.iter()
            .enumerate()
            .fold(nums[..3].iter().sum(), |mut acc, (i, num)| {
                let mut l = i + 1;
                let mut r = nums.len() - 1;
                while l < r {
                    let total = num + nums[l] + nums[r];
                    if (target - total).abs() < (target - acc).abs() {
                        acc = total;
                    }
                    if total > target {
                        r -= 1;
                    } else {
                        l += 1;
                    }
                }
                acc
            })
    }
}

fn main() {
    println!("{}", Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
}
