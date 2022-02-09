package hot_100

func maximalRectangle(matrix [][]byte) int {
	if len(matrix) == 0 {
		return 0
	}
	height := make([]int, len(matrix[0]))
	maxArea := 0
	for i := 0; i < len(matrix); i++ {
		for j := 0; j < len(matrix[0]); j++ {
			if matrix[i][j] == '1' {
				height[j] += 1
			} else {
				height[j] = 0
			}
		}
		if v := largestRectangleArea(height); v > maxArea {
			maxArea = v
		}
	}
	return maxArea
}
