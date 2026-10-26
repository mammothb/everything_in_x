struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut curr = String::new();
        Self::dfs(0, n, &mut curr, &mut result);
        result
    }

    fn dfs(mut left: i32, mut right: i32, curr: &mut String, result: &mut Vec<String>) {
        if left == 0 && right == 0 {
            result.push(curr.clone());
            return;
        }
        if left > 0 {
            curr.push(')');
            Self::dfs(left - 1, right, curr, result);
            curr.pop();
        }
        if right > 0 {
            curr.push('(');
            Self::dfs(left + 1, right - 1, curr, result);
            curr.pop();
        }
    }
}

fn main() {}
