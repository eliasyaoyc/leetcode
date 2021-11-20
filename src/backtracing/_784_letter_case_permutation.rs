struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let mut ret = vec![];
        fn helper(s: &[u8], index: usize,cur: &mut Vec<u8>,ret: &mut Vec<String>) {
            if cur.len() == s.len(){
                ret.push(String::from_utf8(cur.clone()).unwrap());
                return;
            }
            match s[index].is_ascii_digit(){
                true => {
                    cur.push(s[index]);
                    helper(s,index + 1, cur, ret);
                    cur.pop();
                }
                false => {
                    cur.push(s[index]);
                    helper(s,index + 1, cur, ret);
                    cur.pop();

                    if s[index].is_ascii_uppercase() {
                        cur.push(s[index].to_ascii_lowercase());
                        helper(s,index + 1, cur, ret);
                        cur.pop();
                    }else {
                        cur.push(s[index].to_ascii_uppercase());
                        helper(s,index + 1, cur, ret);
                        cur.pop();
                    }
                }
            }
        }
        helper(&s,0,&mut vec![], &mut ret);
        ret
    }
}