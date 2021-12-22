package leetcode_go

func lastStoneWeightII(stones []int) int {
	dp := make([]int, 15001)
	sum := 0
	for _, v := range stones {
		sum += v
	}
	target := sum / 2
	for _, stone := range stones {
		for i := target; i >= stone; i-- {
			dp[i] = max(dp[i], dp[i-stone]+stone)
		}
	}
	return sum - 2*dp[target]
}
