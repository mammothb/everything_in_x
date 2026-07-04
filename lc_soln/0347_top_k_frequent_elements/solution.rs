struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut counter: HashMap<i32, usize> = HashMap::with_capacity(n);
        for num in nums {
            *counter.entry(num).or_default(1) += 1;
        }
        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n + 1];
        for (num, freq) in counter {
            buckets[freq].push(num);
        }
        let mut result = Vec::with_capacity(k);
        for bucket in buckets.into_iter().rev() {
            for num in bucket {
                result.push(num);
                if result.len() == k {
                    return result;
                }
            }
        }
        result
    }
}

fn main() {}
