struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(nums: &[i32], index: usize, cur: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
            ret.push(cur.clone());
            for i in index..nums.len() {
                cur.push(nums[i]);
                dfs(nums,i + 1, cur,ret);
                cur.pop();
            }
        }
        let mut ret = vec![];
        dfs(&nums,0,&mut vec![],&mut ret);
        ret
    }
}

#[test]
fn test(){
    let ret = Solution::subsets(vec![1,2,3]);
    println!("{:?}",ret);
}