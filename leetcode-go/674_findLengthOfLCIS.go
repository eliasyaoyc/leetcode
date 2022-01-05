package leetcode_go

func findLengthOfLCIS(nums []int) int {
	dp := make([]int, len(nums))
	for i := range dp {
		dp[i] = 1
	}
	ret := 1
	for i := 0; i < len(nums)-1; i++ {
		if nums[i+1] > nums[i] {
			dp[i+1] = dp[i] + 1
			if dp[i+1] > ret {
				ret = dp[i+1]
			}
		}
	}
	return ret
}
