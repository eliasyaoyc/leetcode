package leetcode_go

func containsNearbyDuplicate(nums []int, k int) bool {
	window := make(map[int]struct{})
	for i := 0; i < len(nums); i++ {
		if _, ok := window[nums[i]]; ok {
			return true
		}
		window[nums[i]] = struct{}{}

		if len(window) > k {
			delete(window, nums[i-k])
		}
	}
	return false
}
