struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;
        loop {
            let diff = numbers[l as usize] + numbers[r as usize] - target;
            if diff < 0 {
                l += 1;
            } else if diff > 0 {
                r -= 1;
            } else {
                return vec![l as i32 + 1, r as i32 + 1];
            }
        }
        return vec![];
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}
