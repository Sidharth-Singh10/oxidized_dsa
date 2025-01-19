// https://leetcode.com/problems/number-of-provinces/
use std::collections::VecDeque;
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited = vec![false; n]; 
        let mut count = 0;

        for i in 0..n {
            if !visited[i] {
                count += 1;
                Self::bfs(i, &is_connected, &mut visited);
            }
        }

        count
    }

    fn bfs(start: usize, is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>) {
        let mut queue = VecDeque::new();
        queue.push_back(start);
        visited[start] = true;

        while let Some(curr) = queue.pop_front() {
            for neighbor in 0..is_connected.len() {
                if is_connected[curr][neighbor] == 1 && !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
    }
}
