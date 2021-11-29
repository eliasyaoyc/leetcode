use std::cmp::max;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        fn dfs(s: &str, start: usize, cur: usize, used: &mut HashSet<String>, max: &mut usize) {
            if s.len() == start {
                *max = (*max).max(cur);
            } else {
                for i in start..s.len() {
                    if !used.contains(&s[start..=i]) {
                        used.insert(s[start..=i].to_string());
                        dfs(s, i + 1, cur + 1, used, max);
                        used.remove(&s[start..=i]);
                    }
                }
            }
        }
        let mut used = HashSet::new();
        let mut max = 0_usize;
        dfs(&s, 0, 0, &mut used, &mut max);
        max as i32
    }
}

#[test]
fn test() {
    let ret = Solution::max_unique_split("ababccc".to_string());
    println!("{}", ret);
}