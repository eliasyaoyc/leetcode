struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut dp = vec![vec![0; 2]; prices.len()];
        dp[0][0] = -prices[0];
        for i in 1..prices.len() {
            dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] - prices[i]);
            dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] + prices[i] - fee)
        }
        dp[prices.len() - 1][1]
    }
}