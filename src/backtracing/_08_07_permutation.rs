struct Solution;

impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        fn dfs(s: &[char], cur: &mut String, ret: &mut Vec<String>) {
            if cur.len() == s.len() {
                ret.push(cur.clone());
                return;
            }
            for i in 0..s.len() {
                if cur.contains(s[i]) {
                    continue;
                }
                cur.push(s[i]);
                dfs(s,cur,ret);
                cur.pop();
            }
        }
        let s = s.chars().collect::<Vec<_>>();
        let mut ret = vec![];
        dfs(&s,&mut String::new(),&mut ret);
        ret
    }
}

#[test]
fn test(){
    let ret = Solution::permutation("qwe".to_string());
    println!("{:?}",ret);
}