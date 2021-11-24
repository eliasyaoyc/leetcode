struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        fn helper(nums: &[i32], k: i32, target: i32, cur: i32, begin: usize, used: &mut [bool]) -> bool {
            if k == 0 {
                return true;
            }

            if cur == target {
                return helper(nums, k - 1, target, 0, 0, used);
            }

            for i in (begin..nums.len()).rev() {
                if used[i] {
                    continue;
                }
                if cur + nums[i] > target {
                    continue;
                }
                used[i] = true;
                if helper(nums, k, target, cur + nums[i], i + 1, used) {
                    return true;
                }
                used[i] = false;
            }

            false
        }

        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }
        nums.sort();
        let target = sum / k;
        if nums[nums.len() - 1] > target {
            return false;
        }
        let mut used = vec![false; nums.len()];
        helper(&nums, k, target, 0, 0, &mut used)
    }
}

#[test]
fn test(){
    let ret = Solution::can_partition_k_subsets(vec![2,2,2,2,3,4,5],4);
    println!("{}",ret);
}