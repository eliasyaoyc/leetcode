struct Solution;

impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        fn bfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
            if x < 0 || x >= grid.len() || y < 0 || y >= grid[0].len() || grid[x][y] == 0 {
                return 0;
            }
            let temp = grid[x][y];
            grid[x][y] = 0;
            let up = bfs(grid, x, y - 1);
            let down = bfs(grid, x, y + 1);
            let left = bfs(grid, x - 1, y);
            let right = bfs(grid, x + 1, y);
            let max = up.max(down).max(left).max(right);
            grid[x][y] = temp;
            return grid[x][y] + max;
        }
        if grid.is_empty() {
            return 0;
        }
        let mut ret = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let r = bfs(&mut grid, i, j);
                ret = ret.max(r);
            }
        }
        ret
    }
}