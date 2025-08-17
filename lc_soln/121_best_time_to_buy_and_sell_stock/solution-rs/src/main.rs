struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = i32::MAX;
        prices.into_iter().fold(0, |acc, p| {
            buy = buy.min(p);
            acc.max(p - buy)
        })
    }
}

fn main() {
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
}
