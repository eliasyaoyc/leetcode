package hot_100

// 时间复杂度为 On2
func twoSum(nums []int, target int) []int {
	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			if nums[i]+nums[j] == target {
				return []int{i, j}
			}
		}
	}
	return nil
}

// 时间复杂度为 On1
func twoSum_01(nums []int, target int) []int {
	hash := make(map[int]int)
	for i := 0; i < len(nums); i++ {
		if v, exist := hash[target-nums[i]]; exist {
			return []int{v, i}
		}
		hash[nums[i]] = i
	}
	return nil
}
