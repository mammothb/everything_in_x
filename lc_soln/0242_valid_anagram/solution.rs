use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut counter_s = HashMap::new();
        let mut counter_t = HashMap::new();
        for c in s.chars() {
            *counter_s.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            *counter_t.entry(c).or_insert(0) += 1;
        }
        counter_s == counter_t
    }
}

fn main() {}
