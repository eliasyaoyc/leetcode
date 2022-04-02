package hot_100

import "math"

func findTargetSumWays(nums []int, target int) int {
	sum := 0
	for _, n := range nums {
		sum += n
	}
	if math.Abs(float64(target)) > math.Abs(float64(sum)) {
		return 0
	}
	t := sum*2 + 1
	dp := make([][]int, len(nums))
	for i, _ := range dp {
		dp[i] = make([]int, t)
	}
	if nums[0] == 0 {
		dp[0][sum] = 2
	} else {
		dp[0][sum+nums[0]] = 1
		dp[0][sum-nums[0]] = 1
	}

	for i := 1; i < len(nums); i++ {
		for j := 0; j < t; j++ {
			l, r := 0, 0
			if j-nums[i] >= 0 {
				l = j - nums[i]
			}
			if j+nums[i] < target {
				r = j + nums[i]
			}
			dp[i][j] = dp[i-1][l] + dp[i-1][r]
		}
	}
	return dp[len(nums)-1][sum+target]
}
