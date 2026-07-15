use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut max_len = 0;
        let mut left = 0;
        let mut max_freq = 0;
        let mut freqs = HashMap::new();

        for (right, &c) in chars.iter().enumerate() {
            let count = freqs.entry(c).or_insert(0);
            *count += 1;
            max_freq = max_freq.max(*count);

            // window is invalid when (window_len - max_freq) > k
            while (right - left + 1) as i32 - max_freq > k {
                *freqs.get_mut(&chars[left]).unwrap() -= 1;
                left += 1;
            }

            max_len = max_len.max((right - left + 1) as i32);
        }

        max_len
    }
}

fn main() {}
