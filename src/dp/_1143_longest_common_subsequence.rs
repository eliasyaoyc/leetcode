struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let t1 = text1.chars().collect::<Vec<_>>();
        let t2 = text2.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![0; t2.len() + 1]; t1.len() + 1];
        for i in 1..=t1.len() {
            for j in 1..=t2.len() {
                if t1[i - 1] == t2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[t1.len()][t2.len()]
    }
}

#[test]
fn test() {
    let ret = Solution::longest_common_subsequence("abcde".to_owned(),
                                                   "ace".to_owned());
    assert_eq!(ret, 3);
}