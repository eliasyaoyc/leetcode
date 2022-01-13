struct Solution;

impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[2] = 1;
        for i in 3..n as usize + 1 {
            for j in 1..i - 1 {
                dp[i] = dp[i].max(((i - j) *j).max(dp[i - j] * j))
            }
        }
        dp[n as usize] as i32
    }
}