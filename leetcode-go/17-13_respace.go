package leetcode_go

func respace(dictionary []string, sentence string) int {
	dp := make([]int, len(sentence)+1)
	for i := 1; i < len(sentence)+1; i++ {
		dp[i] = dp[i-1]
		for j := 0; j < len(dictionary); j++ {
			if i < len(dictionary[j]) {
				continue
			}
			if sentence[i-len(dictionary[j]):i] == dictionary[j] {
				dp[i] = max(dp[i-len(dictionary[j])]+len(dictionary[j]), dp[i])
			}
		}
	}
	return len(sentence) - dp[len(sentence)]
}
