//https://leetcode.com/problems/01-matrix/
impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let inf = (m + n) as i32;

        for r in 0..m {
            for c in 0..n {
                if mat[r][c] == 0 {
                    continue;
                }
                let top = if r > 0 { mat[r - 1][c] } else { inf };
                let left = if c > 0 { mat[r][c - 1] } else { inf };
                mat[r][c] = std::cmp::min(top, left) + 1;
            }
        }

        for r in (0..m).rev() {
            for c in (0..n).rev() {
                if mat[r][c] == 0 {
                    continue;
                }
                let bottom = if r + 1 < m { mat[r + 1][c] } else { inf };
                let right = if c + 1 < n { mat[r][c + 1] } else { inf };
                mat[r][c] = std::cmp::min(mat[r][c], std::cmp::min(bottom, right) + 1);
            }
        }

        mat
    }
}
