struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        nums.sort();
        fn helper(nums: &Vec<i32>, index: usize, used: &mut Vec<bool>, path: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
            ret.push(path.clone());
            for i in index..nums.len() {
                if  i > 0 && nums[i] == nums[i - 1] && !used[i - 1]{
                    continue;
                }
                path.push(nums[i]);
                used[i] = true;
                helper(nums,i + 1,used,path,ret);
                path.pop();
                used[i] = false;
            }
        }
        helper(&nums, 0, &mut vec![false; nums.len()], &mut vec![], &mut ret);
        ret
    }
}