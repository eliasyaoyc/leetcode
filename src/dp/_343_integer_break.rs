struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[2] = 1;
        for i in 3..=n as usize {
            for j in 1..i - 1 {
                dp[i] = dp[i].max(((i - j) * j).max(dp[i - j] * j));
            }
        }
        dp[n as usize] as i32
    }
}

#[test]
fn test(){
    let ret = Solution::integer_break(10);
    println!("{}",ret);
}