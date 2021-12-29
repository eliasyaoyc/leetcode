struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn rob_range(nums: &[i32], start: usize) -> i32 {
            let mut dp = vec![0; nums.len()];
            dp[1] = nums[start];
            for i in 2..nums.len() {
                dp[i] = dp[i - 1].max(dp[i - 2] + nums[i - 1 + start])
            }
            return dp[nums.len()];
        }
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return nums[0].max(nums[1]);
        }
        rob_range(&nums, 0).max(rob_range(&nums, 1))
    }
}