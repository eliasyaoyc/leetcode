struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(graph: &[Vec<i32>], index: usize, cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if index == graph.len() - 1 {
                ans.push(cur.to_vec());
                return;
            }
            for n in &graph[index] {
                cur.push(*n);
                dfs(graph, *n as usize, cur, ans);
                cur.pop();
            }
        }
        let mut ans = vec![];
        dfs(&graph,0,&mut vec![0], &mut ans);
        ans
    }
}