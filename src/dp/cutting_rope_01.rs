struct Solution;

impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        if n < 4 {
            return n - 1;
        }
        let m = 1000000007;
        let mut n = n as i64;
        let mut ans = 1i64;
        while n > 4 {
            ans = ans * 3 % m;
            n -= 3;
        }
        (ans * n % m) as i32
    }
}