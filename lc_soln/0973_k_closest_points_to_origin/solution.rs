struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut h = BinaryHeap::<(i32, i32, i32)>::new();
        for point in points {
            let dist = point[0] * point[0] + point[1] * point[1];
            h.push((dist, point[0], point[1]));
            if h.len() > k as usize {
                h.pop();
            }
        }
        let mut result = Vec::with_capacity(k as usize);
        while let Some((dist, x, y)) = h.pop() {
            result.push(vec![x, y]);
        }
        result
    }
}

fn main() {}
