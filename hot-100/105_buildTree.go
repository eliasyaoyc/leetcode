package hot_100

func buildTree(preorder []int, inorder []int) *TreeNode {
	if len(preorder) < 1 || len(inorder) < 1 {
		return nil
	}
	left := findRootIndex(preorder[0], inorder)
	root := &TreeNode{
		Val:   preorder[0],
		Left:  buildTree(preorder[1:left+1], inorder[:left]),
		Right: buildTree(preorder[left+1:], inorder[left+1:]),
	}
	return root
}

func findRootIndex(target int, inorder []int) int {
	for i := 0; i < len(inorder); i++ {
		if target == inorder[i] {
			return i
		}
	}
	return -1
}
