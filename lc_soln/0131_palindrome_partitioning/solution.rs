struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut curr = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        Self::dfs(&chars, 0, &mut curr, &mut result);
        result
    }

    fn dfs(chars: &[char], i: usize, curr: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
        if i >= chars.len() {
            result.push(curr.clone());
            return;
        }
        for j in i..chars.len() {
            if Self::is_palindrome(chars, i, j) {
                curr.push(chars[i..j + 1].iter().collect());
                Self::dfs(chars, j + 1, curr, result);
                curr.pop();
            }
        }
    }

    fn is_palindrome(chars: &[char], mut l: usize, mut r: usize) -> bool {
        while l < r {
            if chars[l] != chars[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}

fn main() {}
