struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 || n == 2 {
            return 1;
        }
        Self::fib(n - 1) + Self::fib(n - 2)
    }

    // 加入 dp table 进行剪枝优化
    pub fn fib1(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut dp_table: Vec<i32> = vec![0; (n + 1) as usize];
        dp_table[1] = 1;
        for i in 2..=n as usize {
            dp_table[i] = dp_table[i - 1] + dp_table[i - 2];
        }
        dp_table[n as usize]
    }


    // 通过状态压缩进行优化
    pub fn fib2(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut p_prev = 0;
        let mut prev = 1;
        for i in 2..=n {
            let tmp = p_prev + prev;
            p_prev = prev;
            prev = tmp;
        }
        prev
    }
}

#[test]
fn test() {
    let ret = Solution::fib2(3);
    println!("{}", ret);
}