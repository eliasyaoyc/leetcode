struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ret = vec![];
        let mut used = vec![false; nums.len()];
        fn helper(nums: &[i32], used: &mut [bool], path: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
            if path.len() == nums.len() {
                ret.push(path.clone());
                return;
            }
            for i in 0..nums.len() {
                if i > 0 && nums[i] == nums[i - 1] && used[i - 1] == false {
                    continue;
                }
                if used[i] == false {
                    used[i] = true;
                    path.push(nums[i]);
                    helper(nums, used, path, ret);
                    path.pop();
                    used[i] = false;
                }
            }
        }
        helper(&nums, &mut used, &mut vec![],&mut ret);
        ret
    }
}