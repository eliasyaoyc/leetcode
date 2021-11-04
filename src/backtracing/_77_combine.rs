struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut path = vec![];
        helper(n, k, 1, &mut path, &mut result);
        fn helper(n: i32, k: i32, index: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if path.len() == k as usize {
                result.push(path.clone());
                return;
            }
            for i in index..=n - (k - path.len() as i32) + 1 {
                path.push(i);
                helper(n, k, i + 1, path, result);
                path.pop();
            }
        }
        result
    }
}

#[test]
fn test(){
    let a = Solution::combine(4,2);
    println!("{:?}",a);
}