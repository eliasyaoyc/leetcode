package hot_100

func pathSum(root *TreeNode, targetSum int) int {
	if root == nil {
		return 0
	}
	var rootSum func(node *TreeNode, targetSum int) int
	rootSum = func(node *TreeNode, targetSum int) (res int) {
		if node == nil {
			return
		}
		val := node.Val
		if val == targetSum {
			res++
		}
		res += rootSum(root.Left, targetSum-val)
		res += rootSum(root.Right, targetSum-val)
		return
	}
	res := rootSum(root, targetSum)
	res += rootSum(root.Left, targetSum)
	res += rootSum(root.Right, targetSum)
	return res
}

// 前缀和
func pathSum1(root *TreeNode, targetSum int) (ans int) {
	preSum := map[int64]int{0: 1}
	var dfs func(*TreeNode, int64)
	dfs = func(node *TreeNode, curr int64) {
		if node == nil {
			return
		}
		curr += int64(node.Val)
		ans += preSum[curr-int64(targetSum)]
		preSum[curr]++
		dfs(node.Left, curr)
		dfs(node.Right, curr)
		preSum[curr]--
		return
	}
	return
}
