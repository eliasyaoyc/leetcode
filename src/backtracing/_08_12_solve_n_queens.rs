struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn is_valid(row: isize, col: isize, chessboard: &mut Vec<Vec<char>>, n: usize) -> bool {
            for i in 0..row {
                if chessboard[i as usize][col as usize] == 'Q' {
                    return false;
                }
            }

            let mut i = row - 1;
            let mut j = col - 1;
            while i >= 0 && j >= 0 {
                if chessboard[i as usize][j as usize] == 'Q' {
                    return false;
                }
                i -= 1;
                j -= 1;
            }

            let mut i = row - 1;
            let mut j = col + 1;
            while i >= 0 && j < n as isize {
                if chessboard[i as usize][j as usize] == 'Q' {
                    return false;
                }
                i -= 1;
                j += 1;
            }
            true
        }
        fn dfs(n: usize, row: usize, chessboard: &mut Vec<Vec<char>>, ans: &mut Vec<Vec<String>>) {
            if row == n {
                ans.push(chessboard.iter().map(|s| {
                    let mut v = String::new();
                    for x in s {
                        v.push(x.clone());
                    }
                    v
                }).collect::<Vec<String>>());
                return;
            }
            for col in 0..n {
                if is_valid(row as isize, col as isize, chessboard, n) {
                    chessboard[row][col] = 'Q';
                    dfs(n, row + 1, chessboard, ans);
                    chessboard[row][col] = '.';
                }
            }
        }
        let mut chessboard = vec![vec!['.'; n as usize]; n as usize];
        let mut ret = vec![];
        dfs(n as usize, 0, &mut chessboard, &mut ret);
        ret
    }
}

#[test]
fn test() {
    let ret = Solution::solve_n_queens(4);
    println!("{:?}",ret);
}