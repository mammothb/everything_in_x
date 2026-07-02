use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn sort_features(features: Vec<String>, responses: Vec<String>) -> Vec<String> {
        let mut counter: HashMap<&str, i32> = HashMap::new();
        for response in &responses {
            let mut seen: HashSet<&str> = HashSet::new();
            for token in response.split_ascii_whitespace() {
                if !seen.contains(token) {
                    counter.entry(token).and_modify(|v| *v += 1).or_insert(1);
                    seen.insert(token);
                }
            }
        }
        let mut features = features;
        features.sort_by(|a, b| {
            let v1 = counter.get(a as &str);
            let v2 = counter.get(b as &str);
            v2.cmp(&v1)
        });
        features
    }
}

fn main() {
    let features: Vec<String> = ["chat", "blog", "photo"]
        .iter()
        .map(|&w| String::from(w))
        .collect();
    let responses: Vec<String> = [
        "photo photo photo plz",
        "blog and photo",
        "chat bloggggggg is cool",
    ]
    .iter()
    .map(|&w| String::from(w))
    .collect();
    println!("{:?}", Solution::sort_features(features, responses));
}
