struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![1; intervals.len()];
        for i in 1..intervals.len() {
            for j in 0..i {
                if intervals[j][0] >= intervals[i][1] {
                    dp[i] = dp[i].max(dp[j] + 1)
                }
            }
        }
        let v = dp.iter().max().unwrap();
        intervals.len() as i32 - v
    }
}