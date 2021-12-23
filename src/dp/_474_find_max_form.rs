use std::cmp::max;

struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1];m as usize + 1];
        for str in strs {
            let mut zero_num = 0;
            let mut one_num = 0;
            for c in str.chars() {
                if c == '0' {
                    zero_num += 1;
                }
            }
            one_num = str.len() - zero_num;

            for i in (zero_num..=m as usize).rev() {
                for j in (one_num..=n as usize).rev() {
                    dp[i][j] = max(dp[i][j], dp[i - zero_num][j - one_num] + 1);
                }
            }
        }
        dp[m as usize][n as usize]
    }
}

#[test]
fn test() {
    let ret = Solution::find_max_form(vec!["10".into(), "0001".into(), "111001".into(), "1".into(), "0".into()], 5, 3);
    println!("{}",ret);
}