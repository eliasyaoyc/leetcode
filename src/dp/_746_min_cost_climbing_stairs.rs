struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        if cost.is_empty() {
            return 0;
        }
        if cost.len() == 1 {
            return cost[0];
        }
        let mut dp = vec![0; cost.len()];
        dp[0] = cost[0];
        dp[1] = cost[1];
        for i in 2..cost.len() {
            dp[i] = dp[i - 1].min(dp[i - 2]) + cost[i];
        }
        dp[cost.len() - 1].min(dp[cost.len() - 2])
    }
}