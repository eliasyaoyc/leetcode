struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut ans = vec![];
        fn helper(k: i32, n: i32, index: i32, ans: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            let sum: i32 = ans.iter().sum();
            if sum > n {
                return;
            }
            if ans.len() == k as usize && sum == n {
                result.push(ans.clone());
                return;
            }

            for i in index..=9 - (k - ans.len() as i32) + 1 {
                ans.push(i);
                helper(k, n, i + 1, ans, result);
                ans.pop();
            }
        }
        helper(k, n, 1, &mut ans, &mut result);
        result
    }
}