struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; (m) as usize]; (n) as usize];
        for i in 0..m {
            dp[0][i as usize] = 1;
        }
        for i in 0..n {
            dp[i as usize][0] = 1;
        }
        for i in 1..n as usize {
            for j in 1..m as usize {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[n as usize - 1][m as usize - 1]
    }
}