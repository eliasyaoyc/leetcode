struct Solution;

impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        fn dfs(s: &[u8], cur: &mut Vec<u8>, used: &mut Vec<bool>, ret: &mut Vec<String>) {
            if cur.len() == s.len() {
                let v = unsafe { String::from_utf8_unchecked(cur.clone()) };
                ret.push(v);
                return;
            }
            for i in 0..s.len() {
                if i > 0 && s[i] == s[i - 1] && used[i - 1] == false {
                    continue;
                }
                if used[i] == false {
                    used[i] = true;
                    cur.push(s[i]);
                    dfs(s, cur, used, ret);
                    cur.pop();
                    used[i] = false;
                }
            }
        }
        let mut s = s.into_bytes();
        s.sort();

        let mut used = vec![false; s.len()];
        let mut ret = vec![];
        dfs(&s, &mut Vec::new(), &mut used, &mut ret);
        ret
    }
}

#[test]
fn test() {
    let s = Solution::permutation("jawaLR".to_string());
    println!("{:?}", s);
}