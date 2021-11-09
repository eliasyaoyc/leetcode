struct Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut ret = vec![];
        fn helper(candidates: &Vec<i32>, target: i32, cur: &mut Vec<i32>, index: i32, ret: &mut Vec<Vec<i32>>) {
            let sum:i32 = cur.iter().sum();
            if sum == target {
                ret.push(cur.clone());
                return;
            }
            for i in index..candidates.len() as i32 {
                let v = candidates[i as usize];
                if sum + v > target {
                    break;
                }
                cur.push(v);
                helper(candidates, target, cur, i, ret);
                cur.pop();
            }
        }
        helper(&candidates,target,&mut vec![],0,&mut ret);
        ret
    }
}