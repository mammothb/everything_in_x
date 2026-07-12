struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n < 3 {
            return 0;
        }
        let mut l = 0;
        let mut r = n - 1;
        let mut lh = height[l];
        let mut rh = height[r];
        let mut result = 0;
        while l < r {
            if lh < rh {
                l += 1;
                lh = lh.max(height[l]);
                result += lh - height[l];
            } else {
                r -= 1;
                rh = rh.max(height[r]);
                result += rh - height[r];
            }
        }
        result
    }
}

fn main() {}
