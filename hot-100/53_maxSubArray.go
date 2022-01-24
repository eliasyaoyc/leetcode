package hot_100

func maxSubArray(nums []int) int {
	dp, ans := make([]int, len(nums)), 0
	dp[0] = nums[0]
	ans = dp[0]
	for i := 1; i < len(nums); i++ {
		dp[i] = max(dp[i-1]+nums[i], nums[i])
		ans = max(ans, dp[i])
	}
	return ans
}

func maxSubArray1(nums []int) int {
	sum, ans := 0, -10000
	for i := 0; i < len(nums); i++ {
		sum = max(sum+nums[i], nums[i])
		ans = max(ans, sum)
	}
	return ans
}
