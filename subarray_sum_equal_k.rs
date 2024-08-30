//https://leetcode.com/problems/subarray-sum-equals-k/
use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {

        let mut mp = HashMap::new(); 
        let mut sum = 0;
        let mut cnt = 0;

        mp.insert(0, 1);  
        for &num in &nums {
            sum += num;

            if let Some(&freq) = mp.get(&(sum - k)) {
                cnt += freq; 
            }

            *mp.entry(sum).or_insert(0) += 1;  
        }

        cnt
        
    }
}
