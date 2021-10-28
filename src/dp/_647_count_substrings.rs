struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut ret = 0;
        for i in (0..s.len()).rev() {
            dp[i][i] = true;
            ret += 1;
            for j in i + 1..s.len() {
                dp[i][j] = s[i] == s[j] && if i + 1 == j { true } else { dp[i + 1][j - 1] };
                ret += if dp[i][j] {
                    1
                } else {
                    0
                };
            }
        }
        ret
    }
}