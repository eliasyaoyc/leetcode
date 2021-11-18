struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut ret = 0;
        fn helper(n: usize, idx: usize, used: &mut Vec<bool>, ret: &mut i32) {
            if idx != n {
                for i in 0..10 {
                    if i == 0 && n > 1 && idx == 1 {
                        continue;
                    }
                    if used[i] {
                        continue;
                    }
                    used[i] = true;
                    *ret += 1;
                    helper(n, idx + 1, used, ret);
                    used[i] = false;
                }
            }
        }
        helper(n as usize, 0, &mut vec![false;10],&mut ret);
        ret
    }
}

#[test]
fn test(){
    let r = Solution::count_numbers_with_unique_digits(2);
    println!("{}",r);
}