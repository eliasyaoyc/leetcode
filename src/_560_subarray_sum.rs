use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix_map:HashMap<i32,i32> = HashMap::new();
        prefix_map.insert(0,1);

        let mut ans = 0;
        let mut prefix = 0;
        for num in nums{
            prefix += num;
            if let Some(count) = prefix_map.get(&(prefix - k)){
                ans += count;
            }
            *prefix_map.entry(prefix).or_default() += 1;
        }
        ans
    }
}