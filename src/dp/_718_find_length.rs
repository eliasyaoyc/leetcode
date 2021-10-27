struct Solution;

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        let mut max = 0_i32;
        for i in 1..=nums1.len() {
            for j in 1..=nums2.len() {
                if nums1[i - 1] == nums2[j - 1] {
                    let v = dp[i - 1][j - 1] + 1;
                    if v > max {
                        max = v;
                    }
                    dp[i][j] = v;
                }
            }
        }
        max
    }
}