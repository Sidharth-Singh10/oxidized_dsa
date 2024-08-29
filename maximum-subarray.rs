//https://leetcode.com/problems/maximum-subarray/
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {

            let mut curr_max:i32 = nums[0];
            let mut ans_max:i32  = nums[0];

            for num in  nums.iter().skip(1)
            {
                
                curr_max = std::cmp::max(*num, curr_max+(*num));

                if curr_max> ans_max
                    {ans_max= curr_max;}

            }

            return ans_max;
        
    }
}
