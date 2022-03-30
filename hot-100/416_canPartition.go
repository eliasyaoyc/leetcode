package hot_100

func canPartition(nums []int) bool {
	sum := 0
	for _, n := range nums {
		sum += n
	}
	if sum%2 != 0 {
		return false
	}
	target := sum / 2
	dp := make([]int, target+1)
	for _, num := range nums {
		for i := target; i >= num; i-- {
			if dp[i] < dp[i-num]+num {
				dp[i] = dp[i-num] + num
			}
		}
	}
	return dp[target] == target
}
