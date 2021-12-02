struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for x in &nums {
            sum |= x;
        }

        let mut ret = 0_i32;
        fn dfs(nums: &[i32], sum: i32,idx: usize, cur:i32,ret:&mut i32) {
            if cur == sum {
                *ret += 1 << (nums.len() - idx);
                return;
            }
            if idx == nums.len() {
                return;
            }
            dfs(nums,sum,idx + 1,cur | nums[idx] ,ret);
            dfs(nums,sum,idx + 1,cur,ret);
        }
        dfs(&nums,sum,0,0,&mut ret);
        ret
    }
}

#[test]
fn test(){
    let ret = Solution::count_max_or_subsets(vec![3,2,1,5]);
    println!("{}",ret);
}
