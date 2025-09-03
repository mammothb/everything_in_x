struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        nums.iter()
            .skip(1)
            .fold((nums[0], 1), |(mut last, mut count), &num| {
                if last == num {
                    count += 1;
                } else {
                    count -= 1;
                    if count == 0 {
                        count = 1;
                        last = num;
                    }
                }
                (last, count)
            })
            .0
    }
}

fn main() {
    println!("Hello, world!");
}
