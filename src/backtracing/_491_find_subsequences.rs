struct Solution;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        fn helper(nums: &[i32], index: usize, path: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
            if path.len() >= 2 {
                ret.push(path.clone());
            }
            let mut arr: [usize; 201] = [0; 201];
            for i in index..nums.len() {
                if !path.is_empty() && nums[i] < *path.last().unwrap()
                    || arr[nums[i] as usize + 100] == 1
                {
                    continue;
                }
                arr[nums[i] as usize + 100] = 1;
                path.push(nums[i]);
                helper(nums,i + 1, path,ret);
                path.pop();
            }
        }
        helper(&nums, 0, &mut vec![], &mut ret);
        ret
    }
}