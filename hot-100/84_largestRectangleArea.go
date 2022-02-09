package hot_100

// 使用 42 题一样的方式 从左找最左且大于当前的高度，从右找最右且大于当前的高度会超时
func largestRectangleArea(heights []int) int {
	if len(heights) == 0 {
		return 0
	}
	stack := make([]int, 0, len(heights))
	heights = append([]int{-1}, heights...)
	heights = append(heights, -1)
	maxArea := 0
	for i := 0; i < len(heights); i++ {
		// 这里表示的就是 height[i] < height[i - 1]
		for len(stack) > 0 && heights[peek(stack)] > heights[i] {
			s, p := pop(stack)
			stack = s
			if area := (i - peek(stack) - 1) * heights[p]; area > maxArea {
				maxArea = area
			}
		}
		stack = append(stack, i)
	}
	return maxArea
}

func peek(stack []int) int {
	return stack[len(stack)-1]
}

func pop(stack []int) ([]int, int) {
	p := stack[len(stack)-1]
	stack = stack[:len(stack)-1]
	return stack, p
}
