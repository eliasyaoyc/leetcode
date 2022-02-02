package hot_100

// 完全背包问题
func wordBreak(s string, wordDict []string) bool {
	wordSet := map[string]bool{}
	for _, word := range wordDict {
		wordSet[word] = true
	}
	dp := make([]bool, len(s)+1)
	dp[0] = true
	for i := 1; i < len(s)+1; i++ {
		for j := 0; j < i; j++ {
			if dp[j] && wordSet[s[j:i]] {
				dp[i] = true
				break
			}
		}
	}
	return dp[len(s)]
}
