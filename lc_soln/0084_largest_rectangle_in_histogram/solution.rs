struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut lefts = vec![-1; heights.len()];
        let mut stack = vec![];
        for (i, &h) in heights.iter().enumerate() {
            while let Some(&j) = stack.last() && heights[j] >= h {
                stack.pop();
            }
            if let Some(&j) = stack.last() {
                lefts[i] = j as i32;
            }
            stack.push(i);
        }
        let mut stack = vec![];
        let mut rights = vec![heights.len() as i32; heights.len()];
        for (i, &h) in heights.iter().enumerate().rev() {
            while let Some(&j) = stack.last() && heights[j] >= h {
                stack.pop();
            }
            if let Some(&j) = stack.last() {
                rights[i] = j as i32;
            }
            stack.push(i);
        }
        let mut result = 0;
        for (i, &h) in heights.iter().enumerate() {
            result = result.max(h * (rights[i] - lefts[i] - 1));
        }
        result
    }
}

fn main() {
}
