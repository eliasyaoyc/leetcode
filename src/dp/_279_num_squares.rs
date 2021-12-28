struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![i32::MAX - 1; n as usize + 1];
        dp[0] = 0;
        let mut j = 1_usize;
        while j * j <= n as usize {
            (j * j..=n as usize).for_each(|i| {
                dp[i] = dp[i].min(dp[i - j * j] + 1);
            });
            j += 1;
        }
        dp[n as usize]
    }
}