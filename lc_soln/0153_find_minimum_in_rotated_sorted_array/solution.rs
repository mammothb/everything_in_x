struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums[0] < nums[nums.len() - 1] {
            return nums[0];
        }
        let mut l = 0i32;
        let mut r = (nums.len() - 1) as i32;
        while l <= r {
            let mid = l + (r - l) / 2;
            if nums[mid as usize] > nums[nums.len() - 1] {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        nums[l as usize]
    }
}

fn main() {}
