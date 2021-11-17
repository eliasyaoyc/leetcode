struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn helper(left: i32, right: i32, cur: &mut String, ret: &mut Vec<String>) {
            if left == 0 && right == 0 {
                ret.push(cur.clone());
                return;
            }
            if left > right {
                return;
            }
            if left > 0 {
                cur.push_str("(");
                helper(left - 1, right, cur, ret);
                cur.pop();
            }

            if right > 0 {
                cur.push_str(")");
                helper(left, right - 1, cur, ret);
                cur.pop();
            }
        }
        if n == 0 {
            return vec![];
        }
        let mut ret = vec![];
        helper(n, n, &mut String::new(), &mut ret);
        ret
    }
}