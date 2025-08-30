struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((nums[0], 1, 1), |(res, curr_min, curr_max), &num| {
                let prev_max = curr_max;
                let curr_max = num.max(num * curr_min).max(num * prev_max);
                let curr_min = num.min(num * curr_min).min(num * prev_max);
                (res.max(curr_max), curr_min, curr_max)
            })
            .0
    }
}

fn main() {
    println!("{}", Solution::max_product(vec![2, 3, -2, 4]));
}
