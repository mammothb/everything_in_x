struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let nr = matrix.len();
        let nc = matrix[0].len();

        let mut t = 0i32;
        let mut b = (nr - 1) as i32;

        let mut row = nr;
        while t <= b {
            let mid = (t + (b - t) / 2) as usize;
            if matrix[mid][0] <= target && target <= matrix[mid][nc - 1] {
                row = mid;
                break;
            }
            if matrix[mid][nc - 1] < target {
                t = (mid + 1) as i32;
            } else {
                b = (mid - 1) as i32;
            }
        }

        if row == nr {
            return false;
        }

        let mut l = 0i32;
        let mut r = (nc - 1) as i32;
        while l <= r {
            let mid = (l + (r - l) / 2) as usize;
            if matrix[row][mid] == target {
                return true;
            }
            if matrix[row][mid] < target {
                l = (mid + 1) as i32;
            } else {
                r = (mid - 1) as i32;
            }
        }
        false
    }
}

fn main() {}
