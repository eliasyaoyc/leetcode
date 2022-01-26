package hot_100

func canJump(nums []int) bool {
	step := 0
	for i := 0; i < len(nums); i++ {
		if i > step {
			return false
		}
		step = max(step, i+nums[i])
	}
	return true
}
