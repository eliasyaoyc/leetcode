struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        fn dfs(nums: &[i32], target: i32, index: usize, sum: i32, ans: &mut i32) {
            if index == nums.len() {
                if sum == target {
                    *ans += 1;
                }
                return;
            }
            dfs(nums, target, index + 1, sum + nums[index], ans);
            dfs(nums, target, index + 1, sum - nums[index], ans);
        }
        let mut ans = 0_i32;
        dfs(&nums, target, 0, 0, &mut ans);
        ans
    }
}