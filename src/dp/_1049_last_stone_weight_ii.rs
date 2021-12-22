struct Solution;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum: i32 = stones.iter().sum();
        let mut dp = vec![0; 15001];
        let target = sum / 2;
        for stone in stones {
            for i in (stone..=target).rev() {
                dp[i as usize] = dp[i as usize].max(dp[(i - stone) as usize] + stone)
            }
        }
        sum - 2 * dp[target as usize]
    }
}

#[test]
fn test() {
    let ret = Solution::last_stone_weight_ii(vec![2,7,4,1,8,1]);
    println!("{}", ret)
}