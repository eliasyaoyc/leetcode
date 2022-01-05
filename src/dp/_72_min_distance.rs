struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        for i in 0..word1.len() + 1 {
            dp[i][0] = i;
        }
        for i in 0..word2.len() + 1 {
            dp[0][i] = i;
        }
        for i in 1..word1.len() + 1 {
            for j in 1..word2.len() + 1 {
                if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1].min(dp[i - 1][j - 1])) + 1;
                }
            }
        }
        dp[word1.len()][word2.len()] as i32
    }
}

#[test]
fn test() {
    let ret = Solution::min_distance("horse".to_owned(),
                                     "ros".to_owned());
    println!("{}", ret);
}