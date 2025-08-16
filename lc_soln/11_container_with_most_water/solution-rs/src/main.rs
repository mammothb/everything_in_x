struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut result = 0;
        while l < r {
            result = result.max(height[l].min(height[r]) * (r - l) as i32);
            if height[l] > height[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }
        result
    }
}

fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}
