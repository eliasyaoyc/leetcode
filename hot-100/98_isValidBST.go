package hot_100

func isValidBST(root *TreeNode) bool {
	var prev *TreeNode
	var inorder func(node *TreeNode) bool
	inorder = func(node *TreeNode) bool {
		if node == nil {
			return true
		}
		leftRes := inorder(node.Left)
		if prev != nil && node.Val <= prev.Val {
			return false
		}
		prev = node
		rightRes := inorder(node.Right)
		return leftRes && rightRes
	}
	return inorder(root)
}
