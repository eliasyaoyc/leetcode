package hot_100

func flatten(root *TreeNode) {
	if root == nil {
		return
	}
	stack := []*TreeNode{root}
	var prev *TreeNode
	for len(stack) > 0 {
		cur := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if prev != nil {
			prev.Left, prev.Right = nil, cur
		}
		left, right := cur.Left, cur.Right
		if right != nil {
			stack = append(stack, right)
		}
		if left != nil {
			stack = append(stack, left)
		}
		prev = cur
	}
}
