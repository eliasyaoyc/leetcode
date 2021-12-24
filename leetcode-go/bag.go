package leetcode_go

// 01 bag.
func bag01(weight, value []int, bagWeight int) int {
	dp := make([][]int, len(weight))
	for i, _ := range dp {
		dp[i] = make([]int, bagWeight+1)
	}
	for j := bagWeight; j >= weight[0]; j-- {
		dp[0][j] = dp[0][j-weight[0]] + value[0]
	}

	for i := 1; i < len(weight); i++ {
		for j := weight[i]; j <= bagWeight; j++ {
			dp[i][j] = max(dp[i-1][j], dp[i-1][j-weight[i]]+value[i])
		}
	}
	return dp[len(weight)-1][bagWeight]
}

// 01 bag(一维数组).
func bag011(weight, value []int, bagWeight int) int {
	dp := make([]int, bagWeight+1)
	for i := 0; i < len(weight); i++ {
		for j := bagWeight; j >= weight[i]; j-- {
			dp[j] = max(dp[j], dp[j-weight[i]])
		}
	}
	return dp[bagWeight]
}

// Complete bag.
func bagComplete(weight, value []int, bagWeight int) int {
	dp := make([]int, bagWeight+1)
	for i := 0; i < len(weight); i++ {
		for j := weight[i]; j <= bagWeight; j++ {
			dp[j] = max(dp[j], dp[j-weight[i]]+value[i])
		}
	}
	return dp[bagWeight]
}
