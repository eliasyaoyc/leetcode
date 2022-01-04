struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 5]; prices.len()];
        dp[0][1] = -prices[0];
        dp[0][3] = -prices[0];

        for i in 1..prices.len() {
            dp[i][1] = dp[i - 1][1].max(-prices[i]);
            dp[i][2] = dp[i - 1][2].max(dp[i][1] + prices[i]);
            dp[i][3] = dp[i - 1][3].max(dp[i][2] - prices[i]);
            dp[i][4] = dp[i - 1][4].max(dp[i][3] + prices[i]);
        }
        dp[prices.len() - 1][4]
    }
}