//https://leetcode.com/problems/rotate-image
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let row = matrix.len();

        for i in 0..row {
            for j in 0..=i {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }

        for i in 0..row {
            matrix[i].reverse();
        }
        
    }
}
