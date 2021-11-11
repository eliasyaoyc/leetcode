struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ret = vec![];

        fn is_valid(s: &str) -> bool {
            if s.chars().nth(0) == Some('0') && s.len() > 1 {
                return false;
            }
            match s.parse::<u8>() {
                Ok(_) => true,
                _ => false,
            }
        }

        fn helper(s: &str, cur: &mut Vec<String>, ret: &mut Vec<String>) {
            if s.len() > 12 - cur.len() * 3 || s.len() < 4 - cur.len() {
                return;
            }
            if cur.len() == 3 && is_valid(s){
                cur.push(s[..].to_string());
                ret.push(cur.join(".").to_string());
                cur.pop();
                return;
            }
            for end in 1..s.len(){
                if !is_valid(&s[0..end]){
                    break;
                }
                cur.push(s[..end].to_string());
                helper(&s[end..],cur,ret);
                cur.pop();
            }
        }
        helper(&s,&mut vec![], &mut ret);
        ret
    }
}