struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut result: Vec<i32> = Vec::new();
        let mut start = 0;
        for (i, &num) in nums.iter().enumerate() {
            while let Some(&b) = q.back() && nums[b] < num {
                q.pop_back();
            }
            q.push_back(i);
            if let Some(&f) = q.front() && start > f {
                q.pop_front();
            }
            if (i - start + 1) as i32 >= k {
                result.push(nums[q[0]]);
                start += 1;
            }
        }
        result
    }
}

fn main() {
}
