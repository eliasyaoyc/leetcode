struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut dp = vec![];
        dp[0] = 1;
        let mut l2 = 0;
        let mut l3 = 0;
        let mut l5 = 0;
        for i in 1..n as usize {
            dp[i] = ((dp[l2] * 2).min(dp[l3] * 3)).min(dp[l5] * 5);
            if dp[i] == l2 * 2 {
                l2 += 1;
            }
            if dp[i] == l3 * 3 {
                l3 += 1;
            }
            if dp[i] == l5 * 5 {
                l5 += 1;
            }
        }
        dp[n as usize - 1] as i32
    }
}