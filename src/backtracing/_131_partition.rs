struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        let s: Vec<char> = s.chars().collect();
        Solution::backtracking(&s, &mut res, &mut vec![], 0);
        res
    }

    fn backtracking(s: &Vec<char>, res: &mut Vec<Vec<String>>, path: &mut Vec<String>, start_index: usize) {
        if start_index >= s.len() {
            res.push(path.clone());
            return;
        }
        for i in start_index..s.len() {
            match checked(s, start_index, i) {
                true => {
                    let str: String = s[start_index..i + 1].into_iter().collect();
                    path.push(str);
                    Solution::backtracking(s, res, path, i + 1);
                    path.pop();
                },
                _ => continue,
            }
        }

        fn checked(s: &Vec<char>, start: usize, end: usize) ->bool {
            let (mut start, mut end) = (start, end);
            while start < end {
                if s[start] != s[end] {
                    return false;
                }
                start += 1;
                end -= 1;
            }

            true
        }
    }
}