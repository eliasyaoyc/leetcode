package hot_100

func isSymmetric(root *TreeNode) bool {
	var backtrack func(leftNode, rightNode *TreeNode) bool
	backtrack = func(leftNode, rightNode *TreeNode) bool {
		if leftNode == nil && rightNode == nil {
			return true
		}
		if leftNode == nil || rightNode == nil {
			return false
		}
		return leftNode.Val == rightNode.Val && backtrack(leftNode.Right, rightNode.Left) && backtrack(leftNode.Left, rightNode.Right)
	}
	return backtrack(root.Right, root.Left)
}
