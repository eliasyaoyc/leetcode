struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max = i32::MIN;
        for i in 0..nums.len() {
            sum = (sum + nums[i]).max(nums[i]);
            if sum > max {
                max = sum;
            }
        }
        max
    }
}

#[test]
fn test() {
    let v = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let ret = Solution::max_sub_array(v);
    println!("{}", ret);
}