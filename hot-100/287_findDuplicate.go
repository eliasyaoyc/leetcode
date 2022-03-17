package hot_100

import s "sort"

func findDuplicate(nums []int) int {
	set := map[int]struct{}{}
	for _, n := range nums {
		if _, ok := set[n]; ok {
			return n
		} else {
			set[n] = struct{}{}
		}
	}
	return -1
}

func findDuplicate1(nums []int) int {
	s.Slice(nums, func(i, j int) bool {
		return nums[i] < nums[j]
	})
	slow, fast := 0, 1
	for i := fast; i < len(nums); i++ {
		if nums[i] == nums[slow] {
			return nums[i]
		}
		slow++
		fast++
	}
	return -1
}
