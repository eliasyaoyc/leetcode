struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn helper(candidates: &Vec<i32>, target: i32, index: i32, cur: &mut Vec<i32>, used: &mut Vec<bool>, ret: &mut Vec<Vec<i32>>) {
            let sum: i32 = cur.iter().sum();
            if sum == target {
                ret.push(cur.clone());
                return;
            }
            for i in index..candidates.len() as i32 {
                if sum + candidates[i as usize] > target {
                    break;
                }
                if i > 0 && candidates[i as usize] == candidates[i as usize - 1]&& !used[i as usize - 1] {
                    continue;
                }
                used[i as usize] = true;
                cur.push(candidates[i as usize]);
                helper(candidates, target, i + 1, cur, used, ret);
                used[i as usize] = false;
                cur.pop();
            }
        }
        let mut ret = vec![];
        let mut used = vec![false;candidates.len()];
        candidates.sort();
        helper(&candidates, target, 0, &mut vec![], &mut used, &mut ret);
        ret
    }
}