package hot_100

import "sort"

func threeSum(nums []int) [][]int {
	var ans [][]int
	if len(nums) < 3 {
		return ans
	}
	sort.Ints(nums)
	for i := 0; i < len(nums)-2; i++ {
		if nums[i] > 0 {
			break
		}
		if i >= 1 && nums[i] == nums[i-1] {
			continue
		}
		l, r := i+1, len(nums)-1
		for l < r {
			if nums[i]+nums[l]+nums[r] == 0 {
				ans = append(ans, []int{nums[i], nums[l], nums[r]})
				for l < r && nums[l] == nums[l+1] {
					l++
				}
				for l < r && nums[r] == nums[r-1] {
					r--
				}
				l++
				r--
			} else if nums[i]+nums[l]+nums[r] > 0 {
				r--
			} else {
				l++
			}
		}
	}
	return ans
}
