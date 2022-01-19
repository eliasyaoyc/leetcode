struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![0; 2]; s.len()];
        if s[0] == '0' {
            dp[0][0] = 0;
            dp[0][1] = 1;
        } else {
            dp[0][0] = 1;
            dp[0][1] = 0;
        }
        for i in 1..s.len() {
            if s[i] != 0 {
                dp[i][0] = dp[i - 1][0] + 1;
                dp[i][1] = dp[i - 1][0].min(dp[i - 1][1])
            }else {
                dp[i][0]  = dp[i - 1][0];
                dp[i][1] = dp[i - 1][0].min(dp[i - 1][1]) + 1
            }

        }
        dp[s.len() - 1][0].min(dp[s.len() - 1][1])
    }
}