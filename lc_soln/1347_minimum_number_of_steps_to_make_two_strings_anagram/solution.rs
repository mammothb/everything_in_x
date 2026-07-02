use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut counter_s: HashMap<char, i32> = HashMap::new();
        let mut counter_t: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *counter_s.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            *counter_t.entry(c).or_insert(0) += 1;
        }
        counter_s.iter().fold(0, |acc, (c, v)| {
            acc + (v - counter_t.get(c).copied().unwrap_or(0)).max(0)
        })
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_steps(String::from("bab"), String::from("aba"))
    );
}
