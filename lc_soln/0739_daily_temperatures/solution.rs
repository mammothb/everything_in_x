struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; temperatures.len()];
        let mut stack = vec![];
        for i in 0..temperatures.len() {
            while let Some(&j) = stack.last() && temperatures[i] > temperatures[j] {
                result[j] = (i - j) as i32;
                stack.pop();
            }
            stack.push(i);
        }
        result
    }
}

fn main() {
}
