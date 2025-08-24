use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort_unstable_by_key(|e| e[0]);

        let mut h: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut day = events[0][0];
        let mut i = 0;
        let n = events.len();
        let mut result = 0;
        while i < n || !h.is_empty() {
            while let Some(&Reverse(end_day)) = h.peek()
                && end_day < day
            {
                h.pop();
            }
            while i < n && events[i][0] == day {
                h.push(Reverse(events[i][1]));
                i += 1
            }
            if h.pop().is_some() {
                result += 1;
            }
            day += 1;
        }
        result
    }
}

fn main() {
    let events = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
    println!("{}", Solution::max_events(events));
}
