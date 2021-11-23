
struct Solution;

impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        fn helper(num: &[u8], ret: &mut Vec<i32>) -> bool {
            if num.is_empty() {
                return ret.len() >= 3;
            }
            let n = if num[0] == b'0' {
                1
            } else {
                num.len()
            };
            let mut cur_num = 0_i64;
            let target = if ret.len() >= 2 {
                Some(ret[ret.len() - 1] + ret[ret.len() - 2])
            } else {
                None
            };
            for i in 0..n {
                cur_num = cur_num * 10 + (num[i] - b'0') as i64;
                if cur_num > i32::MAX as i64 {
                    break;
                }

                if target.is_none() || Some(cur_num as i32) == target {
                    ret.push(cur_num as i32);
                    if helper(&num[i + 1..], ret) {
                        return true;
                    }
                    ret.pop();
                } else if Some(cur_num as i32) > target {
                    break;
                }
            }
            false
        }
        let mut ret = vec![];
        helper(&num.into_bytes(), &mut ret);
        ret
    }
}

#[test]
fn test() {
    let ret = Solution::split_into_fibonacci("123456579".to_string());
    println!("{:?}", ret);
}