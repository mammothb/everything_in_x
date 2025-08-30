struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut iter = s.chars().filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        });
        while let (Some(l), Some(r)) = (iter.next(), iter.next_back()) {
            if l != r {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_palindrome(String::from("A man, a plan, a canal: Panama"))
    );
}
