struct Solution;

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let powers: Vec<i32> = (0..32)
            .filter_map(|i| {
                let p = 1 << i;
                if (p & n) != 0 { Some(p) } else { None }
            })
            .collect();
        queries
            .iter()
            .map(|q| {
                let (start, end) = (q[0] as usize, q[1] as usize);
                powers[start..=end]
                    .iter()
                    .fold(1i64, |acc, &p| acc * p as i64 % 1_000_000_007) as i32
            })
            .collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::product_queries(15, vec![vec![0, 1], vec![2, 2], vec![0, 3]])
    );
    println!("{:?}", Solution::product_queries(2, vec![vec![0, 0]]));
}
