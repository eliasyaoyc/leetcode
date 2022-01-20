struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let m = s1.len();
        let n = s2.len();
        if m + n != s3.len() {
            return false;
        }
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        for i in 1..m + 1 {
            if dp[i - 1][0] && s1[i - 1] == s3[i - 1] {
                dp[i][0] = true;
            }
        }
        for i in 1..n + 1 {
            if dp[0][i - 1] && s2[i - 1] == s3[i - 1] {
                dp[0][i] = true;
            }
        }
        for i in 1..m + 1 {
            for j in 1..n + 1 {
                println!("{}",dp[i - 1][j] && s3[i + j - 1] == s1[i- 1]);
                println!("{}",dp[i][j - 1] && s3[i + j - 1] == s2[j - 1]);
                dp[i][j] = (dp[i - 1][j] && s3[i + j - 1] == s1[i- 1]) || (dp[i][j - 1] && s3[i + j - 1] == s2[j - 1]);
            }
        }
        dp[m][n]
    }


    pub fn is_interleave1(s1: String, s2: String, s3: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if s3.len() != n + m {
            return false;
        }
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        // dp[i][j] 表示 s1[..=i] 和 s2[..=j] 是否能构成 s3[..=i + j]
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        //初始化 dp
        for i in 0..n {
            dp[0][i + 1] = dp[0][i] && s2[i] == s3[i];
        }
        for i in 0..m {
            dp[i + 1][0] = dp[i][0] && s1[i] == s3[i];
        }
        println!("{:?}",dp);
        let mut cur;
        for i in 0..m {
            for j in 0..n {
                cur = i + j + 1;
                dp[i + 1][j + 1] = (dp[i + 1][j] && s2[j] == s3[cur]) ||
                    (dp[i][j + 1] && s1[i] == s3[cur]);
            }
        }
        return dp[m][n];
    }
}

#[test]
fn test(){
    let ret = Solution::is_interleave("db".to_owned(),"b".to_owned(),"cbb".to_owned());
    println!("{}",ret);
}