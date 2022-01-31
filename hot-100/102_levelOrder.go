package hot_100

func levelOrder(root *TreeNode) [][]int {
	var ans [][]int
	if root == nil {
		return ans
	}
	queue := []*TreeNode{root}
	for i := 0; len(queue) > 0; i++ {
		var tmpQ []*TreeNode
		var tmpAns []int
		for j := 0; j < len(queue); j++ {
			node := queue[j]
			tmpAns = append(tmpAns, node.Val)
			if node.Left != nil {
				tmpQ = append(tmpQ, node.Left)
			}
			if node.Right != nil {
				tmpQ = append(tmpQ, node.Right)
			}
		}
		queue = tmpQ
	}
	return ans
}
