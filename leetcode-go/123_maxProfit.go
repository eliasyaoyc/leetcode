package leetcode_go

func maxProfit3(prices []int) int {
	dp := make([][]int, len(prices))
	for i := range dp {
		dp[i] = make([]int, 5)
	}
	dp[0][1] = -prices[0]
	dp[0][3] = -prices[0]
	for i := 1; i < len(prices); i++ {
		dp[i][1] = max(dp[i-1][1], -prices[i])
		dp[i][2] = max(dp[i-1][2], dp[i][1]+prices[i])
		dp[i][3] = max(dp[i-1][3], dp[i][2]-prices[i])
		dp[i][4] = max(dp[i-1][4], dp[i][3]+prices[i])
	}
	return dp[len(prices)-1][4]
}
