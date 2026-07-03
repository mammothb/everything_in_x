struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut variants: HashMap<[i32; 26], Vec<String>> = HashMap::with_capacity(strs.len());
        for s in strs {
            let mut chars: [i32; 26] = [0; 26];
            for c in s.chars() {
                let idx = (c as usize) - ('a' as usize);
                chars[idx] += 1;
            }
            variants.entry(chars).or_default().push(s);
        }
        variants.into_values().collect()
    }
}

fn main() {}
