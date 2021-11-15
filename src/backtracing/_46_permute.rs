struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        fn helper(nums: &[i32], path: &mut Vec<i32>, used: &mut [bool], ret: &mut Vec<Vec<i32>>) {
            if path.len() == nums.len() {
                ret.push(path.clone());
                return;
            }

            for i in 0..nums.len() {
                if used[i] {
                    continue;
                }
                used[i] = true;
                path.push(nums[i]);
                helper(nums, path, used, ret);
                path.pop();
                used[i] = false;
            }
        }
        let mut used = vec![false; nums.len()];
        helper(&nums, &mut vec![], &mut used, &mut ret);
        ret
    }
}