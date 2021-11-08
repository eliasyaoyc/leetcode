use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map: HashMap<&str, &str> = [
            ("2", "abc"),
            ("3", "def"),
            ("4", "ghi"),
            ("5", "jkl"),
            ("6", "mno"),
            ("7", "pqrs"),
            ("8", "tuv"),
            ("9", "wxyz")
        ].iter().cloned().collect();

        let mut ans = vec![];
        if digits.len() == 0 {
            return vec![];
        }
        fn helper(curstr: String, next_dig: String, ans: &mut Vec<String>, map: &HashMap<&str, &str>) {
            if next_dig.len() == 0 {
                ans.push(curstr);
                return;
            }
            let dig = &next_dig[0..1];
            let r = map[dig].chars().collect::<Vec<char>>();
            for i in r {
                let mut next_str = curstr.clone();
                next_str.push(i);
                helper(next_str, next_dig[1..].to_string(), ans, map);
            }
        }
        helper("".to_string(), digits, &mut ans, &map);
        ans
    }
}