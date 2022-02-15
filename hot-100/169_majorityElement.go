package hot_100

// 摩尔投票
func majorityElement(nums []int) int {
	candNum, count := nums[0], 1
	for i := 1; i < len(nums); i++ {
		if candNum == nums[i] {
			count += 1
		} else {
			count -= 1
			if count == 0 {
				candNum = nums[i]
				count = 1
			}
		}
	}
	return candNum
}
