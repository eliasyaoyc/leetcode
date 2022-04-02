package hot_100

func convertBST(root *TreeNode) *TreeNode {
	var sum int
	var inOrder func(root *TreeNode)
	inOrder = func(root *TreeNode) {
		if root == nil {
			return
		}
		inOrder(root.Right)
		sum += root.Val
		root.Val = sum
		inOrder(root.Left)
	}
	inOrder(root)
	return root
}
