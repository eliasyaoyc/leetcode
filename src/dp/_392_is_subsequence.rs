struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![0; s.len() + 1]; t.len() + 1];
        for i in 1..t.len() + 1 {
            for j in 1..s.len() + 1 {
                if t[i - 1] == s[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[t.len()][s.len()] == s.len()
    }
}