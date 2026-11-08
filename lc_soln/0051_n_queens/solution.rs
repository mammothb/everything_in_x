struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let n = n as usize;
        let mut curr = vec![vec!['.'; n]; n];
        Self::dfs(0, n, 0, 0, 0, &mut curr, &mut result);
        result
    }

    fn dfs(
        col: usize,
        n: usize,
        rows: u32,
        pos: u32,
        neg: u32,
        curr: &mut Vec<Vec<char>>,
        result: &mut Vec<Vec<String>>,
    ) {
        if col == n {
            result.push(curr.iter().map(|row| row.iter().collect()).collect());
            return;
        }
        for i in 0..n {
            if ((rows >> i) & 1) == 1
                || ((pos >> (i + col)) & 1) == 1
                || ((neg >> (n + i - col)) & 1) == 1
            {
                continue;
            }
            curr[i][col] = 'Q';
            Self::dfs(
                col + 1,
                n,
                rows | (1 << i),
                pos | (1 << (i + col)),
                neg | (1 << (n + i - col)),
                curr,
                result,
            );
            curr[i][col] = '.';
        }
    }
}

fn main() {}
