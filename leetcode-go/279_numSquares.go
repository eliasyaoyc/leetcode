package leetcode_go

import "math"

func numSquares(n int) int {
	dp := make([]int, n+1)
	for i := 0; i < n+1; i++ {
		dp[i] = math.MaxInt32
	}
	dp[0] = 0
	// 物品
	for i := 1; i <= n; i++ {
		// 背包
		for j := i * i; j <= n; j++ {
			dp[j] = min(dp[j], dp[j-i*i]+1)
		}
	}
	return dp[n]
}
