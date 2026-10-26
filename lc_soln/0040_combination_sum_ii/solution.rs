struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut result = Vec::new();
        let mut curr = Vec::new();
        Self::dfs(
            &candidates,
            0,
            candidates.len(),
            target,
            &mut curr,
            &mut result,
        );
        result
    }

    fn dfs(
        nums: &[i32],
        start: usize,
        stop: usize,
        mut target: i32,
        curr: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(curr.clone());
            return;
        }
        if start >= stop {
            return;
        }
        for i in start..stop {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            target -= nums[i];
            if target < 0 {
                return;
            }
            curr.push(nums[i]);
            Self::dfs(nums, i + 1, stop, target, curr, result);
            curr.pop();
            target += nums[i];
        }
    }
}

fn main() {}
