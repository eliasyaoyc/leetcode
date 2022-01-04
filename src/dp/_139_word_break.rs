use std::collections::HashMap;
use std::os::unix::thread::RawPthread;

struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false;s.len() + 1];
        dp[0] = true;
        for i in 1..=s.len(){
            for j in 0..i {
                if dp[j] && word_dict.contains(&s[j..i].to_string()){
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[s.len()]
    }
}