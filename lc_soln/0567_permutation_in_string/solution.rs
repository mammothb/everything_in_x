use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let mut counter: HashMap<char, i32> = HashMap::new();
        for c in s1.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        let mut curr: HashMap<char, i32> = HashMap::new();
        let mut start = 0;
        let chars: Vec<_> = s2.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            if !counter.contains_key(&c) {
                // char not in s1 — reset everything
                curr.clear();
                start = i + 1;
                continue;
            }
            *curr.entry(c).or_insert(0) += 1;
            while *curr.get(&c).unwrap() > *counter.get(&c).unwrap() {
                let left_char = chars[start];
                *curr.get_mut(&left_char).unwrap() -= 1;
                start += 1;
            }
            if i - start + 1 == s1.len() {
                return true;
            }
        }
        false
    }
}

fn main() {}
