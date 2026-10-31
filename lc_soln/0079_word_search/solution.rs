struct Solution;

static DIRS: &'static [i32] = &[0, 1, 0, -1, 0];

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let nr = board.len() as i32;
        let nc = board[0].len() as i32;
        let chars: Vec<char> = word.chars().collect();
        for i in 0..nr {
            for j in 0..nc {
                if Self::dfs(&mut board, i, j, 0, &chars) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: i32, j: i32, pos: usize, word: &[char]) -> bool {
        if pos == word.len() {
            return true;
        }
        let nr = board.len() as i32;
        let nc = board[0].len() as i32;
        if !(0 <= i && i < nr && 0 <= j && j < nc) {
            return false;
        }
        let i_pos = i as usize;
        let j_pos = j as usize;
        let c = board[i_pos][j_pos];
        if c == '.' || c != word[pos] {
            return false;
        }
        board[i_pos][j_pos] = '.';
        for k in 0..4 {
            let next_i = i + DIRS[k];
            let next_j = j + DIRS[k + 1];
            if Self::dfs(board, next_i, next_j, pos + 1, word) {
                return true;
            }
        }
        board[i_pos][j_pos] = c;
        false
    }
}

fn main() {}
