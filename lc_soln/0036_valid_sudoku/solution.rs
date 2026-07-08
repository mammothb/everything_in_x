struct Solution;

const SIZE: usize = 9;
const BSIZE: usize = 3;

fn blk_idx(i: usize, j: usize) -> usize {
    (i / BSIZE) * BSIZE + j / BSIZE
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: [i32; SIZE] = [0; SIZE];
        let mut cols: [i32; SIZE] = [0; SIZE];
        let mut blks: [i32; SIZE] = [0; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                if board[i][j] == '.' {
                    continue;
                }
                let offset = 1 << (board[i][j] as i32 - '0' as i32);
                if (rows[i] & offset) != 0
                    || (cols[j] & offset) != 0
                    || (blks[blk_idx(i, j)] & offset) != 0
                {
                    return false;
                }
                rows[i] |= offset;
                cols[j] |= offset;
                blks[blk_idx(i, j)] |= offset;
            }
        }
        true
    }
}

fn main() {}
