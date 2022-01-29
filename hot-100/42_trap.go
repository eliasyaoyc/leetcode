package hot_100

// 按列求
func trap(height []int) int {
	sum := 0
	for i := 1; i < len(height)-1; i++ {
		maxLeft, maxRight := 0, 0
		for j := i - 1; j >= 0; j-- {
			maxLeft = max(maxLeft, height[j])
		}

		for j := i + 1; j < len(height); j++ {
			maxRight = max(maxRight, height[j])
		}
		m := min(maxLeft, maxRight)
		if m > height[i] {
			sum = sum + (m - height[i])
		}
	}
	return sum
}

// dp
func trapDp(height []int) int {
	sum := 0
	maxLeft := make([]int, len(height))
	maxRight := make([]int, len(height))
	for i := 1; i < len(height)-1; i++ {
		maxLeft[i] = max(maxLeft[i-1], height[i-1])
	}
	for i := len(height) - 2; i >= 0; i-- {
		maxRight[i] = max(maxRight[i+1], height[i+1])
	}
	for i := 1; i < len(height)-1; i++ {
		if min(maxLeft[i], maxRight[i]) > height[i] {
			sum += min(maxLeft[i], maxRight[i]) - height[i]
		}
	}
	return sum
}
