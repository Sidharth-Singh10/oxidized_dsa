//https://leetcode.com/problems/rotting-oranges/
use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        let mut queue = VecDeque::new();
        let mut freshes = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 2 {
                    queue.push_back((i, j));
                } else if grid[i][j] == 1 {
                    freshes += 1;
                }
            }
        }

        if freshes == 0 {
            return 0;
        }

        if queue.is_empty() {
            return -1;
        }

        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut time = 0;

        while !queue.is_empty() {
            let mut rotted = false;
            for _ in 0..queue.len() {
                let (x, y) = queue.pop_front().unwrap();

                for &(dx, dy) in &directions {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx >= 0
                        && nx < m as isize
                        && ny >= 0
                        && ny < n as isize
                        && grid[nx as usize][ny as usize] == 1
                    {
                        grid[nx as usize][ny as usize] = 2;
                        freshes -= 1;
                        queue.push_back((nx as usize, ny as usize));
                        rotted = true;
                    }
                }
            }

            if rotted {
                time += 1;
            }
        }

        if freshes > 0 {
            -1
        } else {
            time
        }
    }
}
