package hot_100

import (
	"math"
	s "sort"
)

func findUnsortedSubarray(nums []int) int {
	if s.IntsAreSorted(nums) {
		return 0
	}
	numsSorted := append([]int(nil), nums...)
	s.Ints(numsSorted)
	left, right := 0, len(nums)-1
	for nums[left] == numsSorted[left] {
		left++
	}
	for nums[right] == numsSorted[right] {
		right--
	}
	return right - left + 1
}

func findUnsortedSubarray1(nums []int) int {
	min := math.MaxInt
	max := math.MinInt
	left, right := -1, -1
	for i, num := range nums {
		if max > num {
			right = i
		} else {
			max = num
		}
		if min < nums[len(nums)-i-1] {
			left = len(nums) - i - 1
		} else {
			min = nums[len(nums)-i-1]
		}
	}
	if right == -1 {
		return 0
	}
	return right - left + 1
}
