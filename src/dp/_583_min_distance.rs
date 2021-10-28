struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.chars().collect::<Vec<_>>();
        let word2 = word2.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![0; word1.len() + 1]; word2.len() + 1];
        for i in 0..word1.len() + 1 {
            dp[0][i] = i as i32;
        }
        for i in 0..word2.len() + 1 {
            dp[i][0] = i as i32;
        }

        for i in 1..word2.len() + 1 {
            for j in 1..word1.len() + 1 {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j - 1] + 2.min(dp[i - 1][j] + 1.min(dp[i][j - 1] + 1));
                }
            }
        }
        dp[word2.len()][word1.len()]
    }
}