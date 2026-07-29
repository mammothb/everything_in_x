use std::collections::HashMap;

struct TimeMap {
    data: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        (*self.data.entry(key).or_insert_with(|| vec![])).push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(values) = self.data.get(&key) {
            let mut l = 0i32;
            let mut r = (values.len() - 1) as i32;
            let mut idx = -1;
            while l <= r {
                let mid = l + (r - l) / 2;
                if values[mid as usize].0 <= timestamp {
                    idx = mid;
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
            if idx == -1 {
                "".to_string()
            } else {
                values[idx as usize].1.to_owned()
            }
        } else {
            "".to_string()
        }
    }
}

fn main() {}
