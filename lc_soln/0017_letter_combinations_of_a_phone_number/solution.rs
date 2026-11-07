struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let letters: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut result = Vec::new();
        let digit_chars: Vec<char> = digits.chars().collect();
        Self::dfs(&letters, &digit_chars, 0, "".to_string(), &mut result);
        result
    }

    fn dfs(letters: &[&str], digits: &[char], pos: usize, curr: String, result: &mut Vec<String>) {
        if pos == digits.len() {
            if !curr.is_empty() {
                result.push(curr);
            }
            return;
        }
        let chars = letters[(digits[pos] as u8 - b'2') as usize];
        for c in chars.chars() {
            let new_curr = format!("{}{}", curr, c);
            Self::dfs(letters, digits, pos + 1, new_curr, result);
        }
    }
}

fn main() {}
