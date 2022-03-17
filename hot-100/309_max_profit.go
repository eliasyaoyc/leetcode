package hot_100

func maxProfit01(prices []int) int {
	dp := make([][]int, len(prices))
	for i := range dp {
		dp[i] = make([]int, 4)
	}
	dp[0][0] = -prices[0]
	for i := 1; i < len(prices); i++ {
		// 买入股票
		dp[i][0] = max(dp[i-1][0], max(dp[i-1][1]-prices[i], dp[i-1][3]-prices[i]))
		// 卖出，冻结期
		dp[i][1] = max(dp[i-1][1], dp[i-1][3])
		// 卖出
		dp[i][2] = dp[i-1][0] + prices[i]
		// 冻结期
		dp[i][3] = dp[i-1][2]
	}
	return max(max(dp[len(prices)-1][1], dp[len(prices)-1][2]), dp[len(prices)-1][3])
}
