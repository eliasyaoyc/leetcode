struct Solution;

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        fn helper(n: i32, prev: i32, used: &mut Vec<bool>, ret: &mut i32) {
            if prev > n {
                *ret += 1;
                return;
            }
            for j in 1..=n {
                if !used[j as usize] && (j % prev == 0 || prev % j == 0) {
                    used[j as usize] = true;
                    helper(n, prev + 1, used, ret);
                    used[j as usize] = false;
                }
            }
        }
        if n == 1 {
            return 1;
        }
        let mut ret = 0_i32;
        helper(n, 1, &mut vec![false;n as usize + 1], &mut ret);
        ret
    }
}