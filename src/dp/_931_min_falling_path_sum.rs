struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let len = matrix.len();
        let mut dp: Vec<Vec<i32>> = matrix;
        for i in 1..len {
            for j in 0..len {
                dp[i][j] += *dp[i - 1][(j.max(1) - 1)..(j + 2).min(len)].iter().min().unwrap();
            }
        }
        *dp.last().unwrap().iter().min().unwrap()
    }
}