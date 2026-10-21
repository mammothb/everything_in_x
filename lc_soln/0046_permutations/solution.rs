struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut curr = nums;
        let mut result = Vec::new();
        Self::dfs(&mut curr, 0, &mut result);
        result
    }

    fn dfs(nums: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            result.push(nums.clone());
            return;
        }
        for i in start..nums.len() {
            nums.swap(i, start);
            Self::dfs(nums, start + 1, result);
            nums.swap(i, start);
        }
    }
}

fn main() {}
