struct Solution;

const DIM: usize = 9;
const SDIM: usize = 3;
const NUMS: &str = "123456789";

fn box_idx(i: usize, j: usize) -> usize {
    (i / SDIM) * SDIM + j / SDIM
}

struct Solver {
    rows: [i32; DIM],
    cols: [i32; DIM],
    boxs: [i32; DIM],
    blanks: Vec<(usize, usize)>,
}

impl Solver {
    fn new(board: &[Vec<char>]) -> Self {
        let mut rows = [0; DIM];
        let mut cols = [0; DIM];
        let mut boxs = [0; DIM];
        let mut blanks = Vec::new();

        for (i, row) in board.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == '.' {
                    blanks.push((i, j));
                } else {
                    let offset = (c as u8 - b'1') as usize;
                    rows[i] |= 1 << offset;
                    cols[j] |= 1 << offset;
                    boxs[box_idx(i, j)] |= 1 << offset;
                }
            }
        }

        Self {
            rows,
            cols,
            boxs,
            blanks,
        }
    }

    fn solve(&mut self, board: &mut Vec<Vec<char>>, idx: usize) -> bool {
        if idx == self.blanks.len() {
            return true;
        }
        let (i, j) = self.blanks[idx];
        let k = box_idx(i, j);
        for num in NUMS.chars() {
            let offset = (num as u8 - b'1') as usize;
            if ((self.rows[i] >> offset) & 1)
                | ((self.cols[j] >> offset) & 1)
                | ((self.boxs[k] >> offset) & 1)
                == 1
            {
                continue;
            }
            board[i][j] = num;
            self.rows[i] |= 1 << offset;
            self.cols[j] |= 1 << offset;
            self.boxs[k] |= 1 << offset;
            if self.solve(board, idx + 1) {
                return true;
            }
            board[i][j] = '.';
            self.rows[i] &= !(1 << offset);
            self.cols[j] &= !(1 << offset);
            self.boxs[k] &= !(1 << offset);
        }
        false
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut solver = Solver::new(board);
        solver.solve(board, 0);
    }
}

fn main() {
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    Solution::solve_sudoku(&mut board);
    println!("{board:#?}");
}
