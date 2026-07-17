struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut counter: HashMap<char, i32> = HashMap::new();
        for c in t.chars() {
            *counter.entry(c).or_default() += 1;
        }
        let mut need = counter.len();
        let mut best = usize::MAX;
        let mut best_start = 0;
        let mut best_end = 0;
        let mut start = 0;
        let chars: Vec<_> = s.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            if let Some(v) = counter.get_mut(&c) {
                *v -= 1;
                if *v == 0 {
                    need -= 1;
                }
            }
            while start <= i && counter.get(&chars[start]).map_or(true, |v| *v < 0) {
                if let Some(v) = counter.get_mut(&chars[start]) {
                    *v += 1;
                }
                start += 1;
            }
            if need == 0 && i - start + 1 < best {
                best = i - start + 1;
                best_start = start;
                best_end = i;
            }
        }
        if best == usize::MAX {
            "".to_string()
        } else {
            s[best_start..=best_end].to_string()
        }
    }
}

fn main() {}
