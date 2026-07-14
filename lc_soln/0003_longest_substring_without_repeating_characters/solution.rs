use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut result = 0;
        let mut start = 0;
        let mut counter = HashMap::<char, i32>::new();
        let chars: Vec<_> = s.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            while counter.get(&c).unwrap_or(&0) > &0 {
                *counter.get_mut(&chars[start]).unwrap() -= 1;
                start += 1;
            }
            *counter.entry(c).or_insert(0) += 1;
            result = result.max((i - start + 1) as i32);
        }
        result
    }
}

fn main() {
    let s = "abcabcbb".to_string();
    println!("{}", Solution::length_of_longest_substring(s));
}
