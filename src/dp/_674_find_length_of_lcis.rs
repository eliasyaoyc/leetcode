struct Solution;

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut ret = 1_i32;
        let mut count = 1_i32;
        for i in 0..nums.len() - 1 {
            if nums[i + 1] > nums[i] {
                count += 1;
            } else {
                count = 1;
            }
            if count > ret {
                ret = count;
            }
        }
        ret
    }
}