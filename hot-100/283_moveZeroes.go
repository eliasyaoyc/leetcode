package hot_100

func moveZeroes(nums []int) {
	if len(nums) == 0 {
		return
	}
	var slow int
	for i := 0; i < len(nums); i++ {
		if nums[i] != 0 {
			nums[slow], nums[i] = nums[i], nums[slow]
			slow++
		}
	}
}
