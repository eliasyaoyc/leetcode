struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;
        for i in 0..coins.len() {
            for j in coins[i]..=amount {
                dp[j as usize] += dp[(j - coins[i]) as usize]
            }
        }
        dp[amount as usize]
    }
}