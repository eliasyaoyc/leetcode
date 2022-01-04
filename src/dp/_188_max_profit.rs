struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if k <= 0 || prices.len() <= 0 {
            return 0;
        }
        let mut dp = vec![vec![0; 2 * k as usize + 1]; prices.len()];
        (1..2 * k as usize).into_iter().step_by(2).for_each(|i| {
            dp[0][i] = -prices[0];
        });
        for i in 1..prices.len() {
            (0..2 * k as usize - 1).into_iter().step_by(2).for_each(|j| {
                dp[i][j + 1] = dp[i - 1][j + 1].max(dp[i - 1][j] - prices[i]);
                dp[i][j + 2] = dp[i - 1][j + 2].max(dp[i - 1][j + 1] + prices[i]);
            })
        }
        dp[prices.len() - 1][2 * k as usize]
    }
}

#[test]
fn test() {
    let ret = Solution::max_profit(2, vec![2, 4, 1]);
    println!("{}", ret);
}