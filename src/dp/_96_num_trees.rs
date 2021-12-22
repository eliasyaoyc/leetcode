struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        for i in 1..=n as usize {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }
        dp[n as usize]
    }
}

#[test]
fn test() {
    let ret = Solution::num_trees(3);
    println!("{}", ret)
}