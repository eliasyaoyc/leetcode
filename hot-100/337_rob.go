package hot_100

func rob1(root *TreeNode) int {
	var dfs func(root *TreeNode) []int
	dfs = func(root *TreeNode) []int {
		if root == nil {
			return []int{0, 0}
		}
		left := dfs(root.Left)
		right := dfs(root.Right)
		rotCur := root.Val + left[0] + right[0]
		notRotCur := max(left[0], left[1]) + max(right[0], right[1])
		return []int{rotCur, notRotCur}
	}
	res := dfs(root)
	return max(res[0], res[1])
}
