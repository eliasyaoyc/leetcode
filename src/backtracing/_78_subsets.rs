struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        fn helper(nums: &Vec<i32>, index: usize, path: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
            ret.push(path.clone());
            for i in index..nums.len() {
                path.push(nums[i].clone());
                helper(nums, i + 1, path, ret);
                path.pop();
            }
        }
        helper(&nums, 0, &mut vec![], &mut ret);
        ret
    }
}