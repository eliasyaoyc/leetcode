struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![vec![0; m]; n];
        dp[0][0] = 1 - obstacle_grid[0][0];
        for i in 1..m {
            if obstacle_grid[i][0] == 0 && dp[0][i - 1] == 1 {
                dp[0][i] = 1;
            }
        }
        for i in 1..n {
            if obstacle_grid[0][i] == 0 && dp[i - 1][0] == 1 {
                dp[i][0] = 1;
            }
        }
        for i in 1..n {
            for j in 1..m {
                if obstacle_grid[j][i] == 1 {
                    continue;
                }
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[n - 1][m - 1]
    }
}