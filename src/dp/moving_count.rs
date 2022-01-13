struct Solution;

impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        fn sum(n: usize) -> usize {
            let mut ret = 0;
            let mut n = n;
            while n > 0 {
                ret += n % 10;
                n /= 10;
            }
            ret
        }
        let mut dp = vec![vec![false; n as usize]; m as usize];
        let mut count = 0;
        let k = k as usize;
        for i in 0..m as usize {
            if sum(i) > k {
                break;
            }
            count += 1;
            dp[i][0] = true;
        }

        for i in 1..n as usize {
            if sum(i) > k {
                break;
            }
            count += 1;
            dp[0][i] = true;
        }

        for i in 1..m as usize {
            for j in 1..n as usize {
                if (dp[i - 1][j] && dp[i][j - 1]) && sum(i) + sum(j) <= k {
                    count += 1;
                    dp[i][j] = true;
                }
            }
        }
        count
    }
}

#[test]
fn test() {
    let ret = Solution::moving_count(16, 8, 4);
    println!("{}", ret);
}