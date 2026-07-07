struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut fwd = vec![1; n];
        let mut bwd = vec![1; n];
        for i in 1..n {
            fwd[i] = fwd[i - 1] * nums[i - 1];
            bwd[n - 1 - i] = bwd[n - i] * nums[n - i];
        }
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            result.push(fwd[i] * bwd[i]);
        }
        result
    }
}

fn main() {}
