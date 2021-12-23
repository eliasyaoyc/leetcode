package leetcode_go

func findTargetSumWays(nums []int, target int) int {
	sum := 0
	for _, num := range nums {
		sum += num
	}

	if target > sum || (sum+target)%2 == 1 {
		return 0
	}

	bag := (sum + target) / 2

	dp := make([]int, bag+1)
	dp[0] = 1

	for i := 0; i < len(nums); i++ {
		for j := bag; j >= nums[i]; j++ {
			dp[j] += dp[j-nums[i]]
		}
	}

	return dp[bag]
}
