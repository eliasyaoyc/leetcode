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
                if nums[i][0] > nums[j][0] && nums[i][1] > nums[j][1] && nums[i][2] > nums[j][2] {
                    dp[i] = dp[i].max(dp[j] + nums[i][2])
                }
            }
            ret = max(ret,dp[i])
        }

        ret
    }
}