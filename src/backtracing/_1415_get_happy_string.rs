struct Solution;

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        fn dfs(n: usize, k : usize, cur: &mut Vec<u8>,ret: &mut Vec<String>){
            if ret.len() == k {
                return;
            }
            if cur.len() == n {
                ret.push(cur.iter().map(|&i| (b'a' + i) as char).collect());
                return;
            }else {
                for x in 0..3 {
                    if cur.len() > 0 && x == cur[cur.len() - 1] {
                        continue;
                    }
                    cur.push(x);
                    dfs(n,k,cur,ret);
                    cur.pop();
                }
            }
        }
        let mut ret = vec![];
        dfs(n as usize, k as usize, &mut vec![], &mut ret);
        if ret.len() < k as usize {
            "".to_string()
        }else {
            ret.pop().unwrap()
        }
    }
}

#[test]
fn test() {
    let ret = Solution::get_happy_string(1, 3);
    println!("{}", ret);
}