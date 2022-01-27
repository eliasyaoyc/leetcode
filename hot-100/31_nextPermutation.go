package hot_100

// 123465
// 第一步：从右到左，把大数和小数做替换，这里不是 64换，而是 54 换
// 第二步：123564，替换后要把5之后的数字做升序，123456 这样更小
func nextPermutation(nums []int) {
	if len(nums) <= 1 {
		return
	}
	i, j, k := len(nums)-2, len(nums)-1, len(nums)-1

	for i >= 0 && nums[i] >= nums[j] {
		i--
		j--
	}
	if i >= 0 {
		for nums[i] >= nums[k] {
			k--
		}
		nums[i], nums[k] = nums[k], nums[i]
	}
	for i, j := j, len(nums)-1; i < j; i, j = i+1, j-1 {
		nums[i], nums[j] = nums[j], nums[i]
	}
}
