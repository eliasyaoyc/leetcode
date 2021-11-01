struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums1.len() + 1]; nums2.len() + 1];
        for i in 1..=nums2.len() {
            for j in 1..=nums1.len() {
                if nums2[i - 1] == nums1[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                }else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[nums2.len()][nums1.len()]
    }
}