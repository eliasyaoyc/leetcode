struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let size = (n + 1) as usize;
        let mut dp = vec![0; size];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }

    pub fn climb_stairs1(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        let mut c = 0;
        for i in 1..=n as usize {
            c = a + b;
            a = b;
            b = c;
        }
        c
    }
}