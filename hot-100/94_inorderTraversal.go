package hot_100

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func inorderTraversal(root *TreeNode) []int {
	var ans []int
	var inorder func(root *TreeNode)
	inorder = func(root *TreeNode) {
		if root == nil {
			return
		}
		inorder(root.Left)
		ans = append(ans, root.Val)
		inorder(root.Right)
	}
	inorder(root)
	return ans
}
