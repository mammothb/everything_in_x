struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let first = strs[0].clone();
        for (i, c) in first.char_indices() {
            if strs.iter().skip(1).any(|s| s.chars().nth(i) != Some(c)) {
                return first.clone()[..i].to_string();
            }
        }
        first
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight")
        ])
    );
}
