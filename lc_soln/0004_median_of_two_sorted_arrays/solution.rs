struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let mut n1 = nums1.len() as i32;
        let mut n2 = nums2.len() as i32;
        if n1 > n2 {
            (n1, n2) = (n2, n1);
            (nums1, nums2) = (nums2, nums1);
        }

        let half = (n1 + n2 + 1) / 2;

        let mut l = 0i32;
        let mut r = n1;
        loop {
            let i = l + (r - l) / 2;
            let j = half - i;

            let left1 = if i > 0 {
                nums1[(i - 1) as usize]
            } else {
                i32::MIN
            };
            let right1 = if i < n1 { nums1[i as usize] } else { i32::MAX };
            let left2 = if j > 0 {
                nums2[(j - 1) as usize]
            } else {
                i32::MIN
            };
            let right2 = if j < n2 { nums2[j as usize] } else { i32::MAX };

            if left1 <= right2 && left2 <= right1 {
                if (n1 + n2) % 2 == 0 {
                    return (left1.max(left2) + right1.min(right2)) as f64 / 2.0;
                }
                return left1.max(left2) as f64;
            } else if left1 > right2 {
                r = i - 1;
            } else {
                l = i + 1;
            }
        }
    }
}

fn main() {}
