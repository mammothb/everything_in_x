struct Solution;

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let n = s.len();
        let k = k as usize;
        let s_bytes = s.as_bytes();
        let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX - 101; k + 1]; n + 1];
        dp[0].fill(0);

        for i in 1..=n {
            let c = s_bytes[i - 1];
            for j in 0..=k.min(i) {
                if j > 0 {
                    dp[i][j] = dp[i - 1][j - 1];
                }
                let mut removed = 0;
                let mut count = 0;
                for p in (0..i).rev() {
                    if s_bytes[p] == c {
                        count += 1;
                    } else {
                        removed += 1;
                        if removed > j {
                            break;
                        }
                    }
                    dp[i][j] = dp[i][j].min(dp[p][j - removed] + Self::get_len(count));
                }
            }
        }
        dp[n][k]
    }

    fn get_len(count: i32) -> i32 {
        match count {
            count if count <= 1 => count,
            2..=9 => 2,
            10..=99 => 3,
            _ => 4,
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::get_length_of_optimal_compression(String::from("aaabcccd"), 2)
    );
}
