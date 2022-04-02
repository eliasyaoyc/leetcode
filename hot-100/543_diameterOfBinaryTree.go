package hot_100

func diameterOfBinaryTree(root *TreeNode) int {
	if root == nil {
		return 0
	}
	var ans int
	var dfs func(root *TreeNode) int
	dfs = func(root *TreeNode) int {
		if root == nil {
			return 0
		}
		lc := dfs(root.Left)
		rc := dfs(root.Right)
		ans = max(ans, lc+rc)
		return max(lc, rc) + 1
	}
	dfs(root)
	return ans
}
