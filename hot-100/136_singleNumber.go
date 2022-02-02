package hot_100

import "sort"

// 可以直接使用 hash 的特性来解决，但是需要额外的空间
func singleNumber(nums []int) int {
	sort.Slice(nums, func(i, j int) bool {
		return nums[i] < nums[j]
	})
	slow, fast := 0, 1
	for fast < len(nums) {
		if nums[slow] != nums[fast] {
			return nums[slow]
		}
		slow += 2
		fast += 2
	}
	return nums[len(nums)-1]
}
