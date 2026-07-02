struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .zip(prices.iter().skip(1))
            .fold(0, |acc, (a, b)| acc + std::cmp::max(0, b - a))
    }
}

fn main() {
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
}
