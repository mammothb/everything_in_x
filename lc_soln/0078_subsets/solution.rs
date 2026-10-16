struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut curr = Vec::new();
        Self::dfs(&nums, 0, nums.len(), &mut curr, &mut result);
        result
    }

    fn dfs(
        nums: &[i32],
        start: usize,
        stop: usize,
        curr: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        result.push(curr.clone());
        if start >= stop {
            return;
        }
        for i in start..stop {
            curr.push(nums[i]);
            Self::dfs(nums, i + 1, stop, curr, result);
            curr.pop();
        }
    }
}

fn main() {}
