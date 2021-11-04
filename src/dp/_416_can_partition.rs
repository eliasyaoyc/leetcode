struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return false;
        }
        let n = nums.len();
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let target = sum / 2;
        let mut dp = vec![0; (target + 1) as usize];
        for i in 0..n {
            for j in (nums[i]..=target).rev() {
                dp[j as usize] = dp[j as usize].max(dp[(j - nums[i]) as usize] + nums[i]);
            }
        }
        dp[target as usize] == target
    }
}

#[test]
fn test(){
    let a = Solution::can_partition(vec![1,2,5]);
    assert_eq!(a, false);
}