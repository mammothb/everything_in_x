struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let cap: usize = strs
            .iter()
            .map(|s| s.len() + digit_count(s.len()) + 1)
            .sum();
        let mut result = String::with_capacity(cap);
        for s in &strs {
            result.push_str(&s.len().to_string());
            result.push('#');
            result.push_str(s);
        }
        result
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut result = vec![];
        let mut rest: &str = &s;
        while let Some((lstr, after)) = rest.split_once('#') {
            let length: usize = lstr.parse().unwrap();
            let (s, remainder) = after.split_at(length);
            result.push(s.to_string());
            rest = remainder;
        }
        result
    }
}

fn digit_count(n: usize) -> usize {
    match n.checked_ilog10() {
        Some(log) => log as usize + 1,
        None => 1, // n == 0
    }
}

fn main() {
    let s = Solution::encode(vec!["Hello".to_string(), "World".to_string()]);
    println!("{}", s);
    println!("{:?}", Solution::decode(s));
}
