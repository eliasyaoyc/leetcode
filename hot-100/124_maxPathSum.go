package hot_100

import "math"

func maxPathSum(root *TreeNode) int {
	maxSum := math.MinInt32
	var dfs func(root *TreeNode) int
	dfs = func(root *TreeNode) int {
		if root == nil {
			return 0
		}
		left := dfs(root.Left)
		right := dfs(root.Right)
		// innerMax 是当前node作为root的时候的最大值，也就是说只有在当前节点是root的时候才能够同时选左边和右边；
		// 不然的话，作为路径上的一份子，只能选择左边或者右边。
		// 因为dfs的最顶层是二叉树的root，所以其实我们每个节点当root的可能性都考虑到了，所以选择在innermax那边更新
		if innerMaxSum := root.Val + left + right; innerMaxSum > maxSum {
			maxSum = innerMaxSum
		}
		return max(root.Val+max(left, right), 0)
	}
	dfs(root)
	return maxSum
}
