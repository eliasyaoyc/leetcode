use std::cmp::max;

struct Solution;

impl Solution {
    pub fn pile_box(mut nums: Vec<Vec<i32>>) -> i32 {
        nums.sort_by(|a, b| {
            a[0].cmp(&b[0])
        });
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0][2];
        let mut ret = nums[0][2];

        for i in 1..nums.len() {
            dp[i] = nums[i][2];
            for j in 0..nums.len() {
                if dp[i][0] > dp[j][0] && dp[i][1] > dp[j][1] && dp[i][2] > dp[j][2] {
                    dp[i] = dp[i].max(dp[j] + nums[i][2])
                }
            }
            ret = max(ret,dp[i])
        }

        ret
    }
}