//https://leetcode.com/problems/flood-fill/
use std::collections::VecDeque;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let m = image.len();
        let n = image[0].len();
        let sr = sr as usize;
        let sc = sc as usize;
        let start_color = image[sr][sc];

        if start_color == color {
            return image;
        }

        let dirs = [(0, 1), (1, 0), (-1, 0), (0, -1)];
        let mut q = VecDeque::new();
        q.push_back((sr, sc));

        while !q.is_empty() {
            let c = q.pop_front().unwrap();
            image[c.0][c.1] = color;

            for &(dx, dy) in &dirs {
                let i = c.0 as isize + dx;
                let j = c.1 as isize + dy;

                if i >= 0
                    && i < m as isize
                    && j >= 0
                    && j < n as isize
                    && image[i as usize][j as usize] == start_color
                {
                    q.push_back((i as usize, j as usize));
                }
            }
        }

        image
    }
}
