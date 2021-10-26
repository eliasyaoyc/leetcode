struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let v: i32 = nums.iter().sum();
        let v = v.abs();
        if target.abs() > v {
            return 0;
        }
        let mut dp = vec![vec![0; (v * 2 + 1) as usize]; nums.len() + 1];
        dp[0][v as usize] = 1;
        for i in 0..nums.len(){
            for j in 0..dp[i].len() {
                if dp[i][j] != 0 {
                    dp[i+1][j + nums[i] as usize] += dp[i][j];
                    dp[i+1][j - nums[i] as usize] += dp[i][j];
                }
            }
        }
        dp[dp.len() - 1][(v + target) as usize]
    }
}

#[test]
fn test(){
    let v = Solution::find_target_sum_ways(vec![100],-200);
    println!("{}",v);
}