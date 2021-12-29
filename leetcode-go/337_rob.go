package leetcode_go

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rob3(root *TreeNode) int {
	res := robTree(root)
	return max(res[0], res[1])
}

func robTree(cur *TreeNode) []int {
	if cur == nil {
		return []int{0, 0}
	}
	left := robTree(cur.Left)
	right := robTree(cur.Right)

	rotCur := cur.Val + left[0] + right[0]
	notRotCur := max(left[0], left[1]) + max(right[0], right[1])
	return []int{notRotCur, rotCur}
}
