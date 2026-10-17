struct Solution;

impl Solution {
    pub fn combination_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = Vec::new();
        let mut buf = Vec::new();
        Self::dfs(&nums, 0, nums.len(), 0, &mut buf, &mut result, target);
        result
    }

    fn dfs(
        nums: &[i32],
        start: usize,
        stop: usize,
        mut curr: i32,
        buf: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        target: i32,
    ) {
        if curr == target {
            result.push(buf.clone());
            return;
        }
        if start >= stop {
            return;
        }
        for i in start..stop {
            curr += nums[i];
            if curr > target {
                return;
            }
            buf.push(nums[i]);
            Self::dfs(nums, i, stop, curr, buf, result, target);
            buf.pop();
            curr -= nums[i];
        }
    }
}

fn main() {}
