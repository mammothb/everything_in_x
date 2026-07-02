struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;
        while l < r {
            match numbers[l] + numbers[r] - target {
                diff if diff > 0 => r -= 1,
                diff if diff < 0 => l += 1,
                _ => return vec![l as i32 + 1, r as i32 + 1],
            }
        }
        vec![]
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}
