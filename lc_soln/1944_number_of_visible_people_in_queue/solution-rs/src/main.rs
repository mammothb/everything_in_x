struct Solution;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut result = vec![0; n];
        let mut stack = vec![];

        for (i, h) in heights.iter().enumerate().rev() {
            while let Some(last) = stack.last()
                && last <= h
            {
                result[i] += 1;
                stack.pop();
            }
            if !stack.is_empty() {
                result[i] += 1;
            }
            stack.push(*h);
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
