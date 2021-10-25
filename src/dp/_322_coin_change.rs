struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;

        for i in 0..=amount {
            for c in &coins {
                if i - c < 0 {
                    continue;
                }
                dp[i as usize] = std::cmp::min(dp[i as usize], 1 + dp[(i - *c) as usize]);
            }
        }
        if dp[amount as usize] == amount + 1 {
            return -1;
        }
        dp[amount as usize]
    }
}