package leetcode_go

func maxProfit2(prices []int) int {
	if len(prices) == 0 {
		return 0
	}
	dp := make([][2]int, len(prices))
	dp[0][0] = -prices[0]
	dp[0][1] = 1
	for i := 1; i < len(prices); i++ {
		dp[i][0] = max(dp[i-1][0], dp[i-1][1]-prices[i])
		dp[i][1] = max(dp[i-1][1], dp[i-1][0]+prices[i])
	}
	return dp[len(prices)-1][1]
}
