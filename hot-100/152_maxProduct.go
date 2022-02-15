package hot_100

func maxProduct(nums []int) int {
	dp := make([][]int, len(nums))
	for i := range dp {
		dp[i] = make([]int, 2)
	}
	dp[0][0] = nums[0]
	dp[0][1] = nums[0]
	res := dp[0][1]
	for i := 1; i < len(nums); i++ {
		if nums[i] > 0 {
			dp[i][0] = min(dp[i-1][0]*nums[i], nums[i])
			dp[i][1] = max(dp[i-1][1]*nums[i], nums[i])
		} else {
			dp[i][0] = min(dp[i-1][1]*nums[i], nums[i])
			dp[i][1] = max(dp[i-1][0]*nums[i], nums[i])
		}
		res = max(res, dp[i][1])
	}
	return res
}
