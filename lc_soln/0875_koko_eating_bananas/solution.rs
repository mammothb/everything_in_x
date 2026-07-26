struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1i32;
        let mut r = piles.iter().max().copied().unwrap();
        let mut result = r;
        while l <= r {
            let mid = l + (r - l) / 2;
            let count: i32 = piles.iter().map(|&pile| (pile + mid - 1) / mid).sum();
            if count <= h {
                result = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        result
    }
}

fn main() {}
