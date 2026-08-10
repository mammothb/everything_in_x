struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut fast = nums[0];
        let mut slow = nums[0];
        let mut result = -1;
        loop {
            fast = nums[nums[fast as usize] as usize];
            slow = nums[slow as usize];
            if fast == slow {
                let mut slow2 = nums[0];
                while slow != slow2 {
                    slow = nums[slow as usize];
                    slow2 = nums[slow2 as usize];
                }
                result = slow2;
                break;
            }
        }
        result
    }
}

fn main() {}
