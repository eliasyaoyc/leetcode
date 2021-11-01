struct Solution;
/// s: "baegg"
/// t: "bag"
///  ------------------------------
///          b  |   a   |  g
///  ------------------------------
///   | 1 | 0  |   0   |  0
///  ------------------------------
/// b | 1 | 1  |   0   |  0
///  ------------------------------
/// a | 1 | 1  |   1   |  0
///  ------------------------------
/// e | 1 | 1  |   1   |  0
///  ------------------------------
/// g | 1 | 1  |   1   |  1
///  ------------------------------
/// g | 1 | 1  |   1   |  2
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
        for i in 0..=s.len() {
            dp[i][0] = 1;
        }
        for i in 1..=s.len() {
            for j in 1..t.len() {
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
        dp[s.len()][t.len()]
    }
}