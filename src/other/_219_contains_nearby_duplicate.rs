struct Solution;

impl Solution {
    // 超出时间限制
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] {
                    if i32::abs(i as i32 - j as i32) <= k {
                        return true;
                    }
                }
            }
        }
        false
    }

    // 滑动窗口
    pub fn contains_nearby_duplicate_1(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::with_capacity(k as usize);
        for i in 0..nums.len() {
            if set.contains(&nums[i]) {
                return true;
            }
            set.insert(nums[i]);
            if set.len() > k as usize {
                set.remove(&nums[i - k as usize]);
            }
        }
        false
    }
}