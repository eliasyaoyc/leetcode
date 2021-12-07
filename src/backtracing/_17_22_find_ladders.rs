struct Solution;

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<String> {
        fn diff(from: &String, to: &String) -> bool {
            if from.len() != to.len() {
                return false;
            }
            let mut diff = 0;
            for i in 0..from.len() {
                if from.as_bytes()[i] != to.as_bytes()[i] {
                    diff += 1;
                    if diff > 1 {
                        return false;
                    }
                }
            }
            diff == 1
        }

        fn dfs(word_list: &[String], visited: &mut [bool], ans: &mut Vec<String>, cur: &String, end_word: &String) -> bool {
            if cur == end_word {
                return true;
            }
            for i in 0..word_list.len() {
                if !visited[i] && diff(cur, &word_list[i]) {
                    ans.push(word_list[i].clone());
                    visited[i] = true;
                    if dfs(word_list, visited, ans, &word_list[i], end_word) {
                        return true;
                    }
                    ans.pop();
                }
            }
            false
        }

        if !word_list.contains(&end_word) {
            return vec![];
        }
        let mut visited = vec![false; word_list.len()];
        let mut ans = vec![begin_word.clone()];

        if dfs(&word_list, &mut visited, &mut ans, &begin_word, &end_word) {
            return ans;
        }

        vec![]
    }
}

#[test]
fn test() {
    let ans = Solution::find_ladders("hit".into(), "cog".into(), vec!["hot".into(), "dot".into(), "dog".into(), "lot".into(), "log".into(), "cog".into()]);
    println!("{:?}", ans);
}