struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 0..=target {
            for j in 0..nums.len() {
                if i >= nums[j] {
                    dp[i as usize] += dp[i as usize - nums[j] as usize]
                }
            }
        }
        dp[target as usize]
    }
}