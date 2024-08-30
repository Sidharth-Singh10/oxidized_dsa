//https://leetcode.com/problems/longest-consecutive-sequence
use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        
        //nums is moved to set st
        let st: HashSet<i32> = nums.into_iter().collect();

        let mut len = 0;
        
        // iterating over set st
        for &num in &st {
            if !st.contains(&(num - 1)) {
                let mut curr = 1;
                let mut current_num = num;
                
                while st.contains(&(current_num + 1)) {
                    current_num += 1;
                    curr += 1;
                }

                len = len.max(curr);
            }
        }

        len
        
    }
}
