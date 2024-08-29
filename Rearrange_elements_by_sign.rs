
//https://leetcode.com/problems/rearrange-array-elements-by-sign/
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {

        let mut pos:usize = 0;
        let mut neg:usize = 1;

        let n = nums.len();
        let mut ans = vec![0;n];

         for &nums in nums.iter() {
            if nums > 0 {
                ans[pos] = nums;
                pos += 2;
            } else {
                ans[neg] = nums;
                neg += 2;
            }
        }

        ans        
    }
}
