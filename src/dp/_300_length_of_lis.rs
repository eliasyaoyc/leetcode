struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![1; nums.len()];
        let mut max = 1;
        for i in 0..nums.len() {
            for j in 0..i{
                if nums[i] > nums[j] {
                    let v = std::cmp::max(dp[i], dp[j] + 1);
                    if v > max {
                        max = v;
                    }
                    dp[i] = v;
                }
            }
        }
        max
    }
}

#[test]
fn test(){
    let v = vec![7,7,7,7,7,7];
    let ret = Solution::length_of_lis(v);
    println!("{}",ret);
}