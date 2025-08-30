struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((i32::MAX, 0, i32::MAX, 0), |mut acc, &p| {
                acc.0 = acc.0.min(p);
                acc.1 = acc.1.max(p - acc.0);
                acc.2 = acc.2.min(p - acc.1);
                acc.3 = acc.3.max(p - acc.2);
                acc
            })
            .3
    }
}

fn main() {
    println!("{}", Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
}
