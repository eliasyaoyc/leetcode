struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![0; s.len()]; s.len()];
        for i in (0..s.len()).rev() {
            dp[i][i] = 1;
            for j in i + 1..s.len() {
                if s[i] == s[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[0][s.len() - 1]
    }


    // todo 用状态压缩进行优化
    pub fn longest_palindrome_subseq1(s: String) -> i32 {
        0
    }
}