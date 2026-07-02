struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let (mut l, mut r) = weights.iter().fold((0, 0), |mut acc, w| {
            acc.0 = acc.0.max(*w);
            acc.1 += *w;
            acc
        });
        while l <= r {
            let mid = l + (r - l) / 2;
            if Self::can_ship(&weights, mid, days) {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        l
    }

    fn can_ship(weights: &[i32], capacity: i32, days: i32) -> bool {
        let mut result = 1;
        let mut curr = 0;
        for &w in weights {
            curr += w;
            if curr > capacity {
                result += 1;
                curr = w;
            }
        }
        result <= days
    }
}

fn main() {
    println!(
        "{}",
        Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5)
    );
}
