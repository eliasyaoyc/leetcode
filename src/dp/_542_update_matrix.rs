struct Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = 10000;
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if i as isize - 1 >= 0 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
                }
                if j as isize - 1 >= 0 {
                    dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1)
                }
            }
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i + 1 < m {
                    dp[i][j] = dp[i][j].min(dp[i + 1][j] + 1)
                }
                if j + 1 < n {
                    dp[i][j] = dp[i][j].min(dp[i][j + 1] + 1)
                }
            }
        }
        dp
    }
}