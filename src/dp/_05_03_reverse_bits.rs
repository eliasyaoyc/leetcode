struct Solution;

impl Solution {
    pub fn reverse_bits(mut num: i32) -> i32 {
        if num == 0  {
            return 1;
        }
        let mut max = 0;
        let mut current = 0; // 代表当前连续1的数量，从地位开始
        let mut reserve = 0; // 代表当前反转一次0连续1的数量，从地位开始
        for i in 0..32 {
            if num & 1 == 1 {
                current = current + 1;
                reserve = reserve + 1;
            } else {
                reserve = current + 1;
                current = 0;
            }
            if reserve > max {
                max = reserve;
            }

            num = num >> 1;
        }
        max
    }
}