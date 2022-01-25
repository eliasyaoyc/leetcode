package hot_100

func longestPalindrome(s string) string {
	l := len(s)
	if l < 2 {
		return s
	}
	dp, maxLen, start := make([][]bool, l), 1, 0
	for i := range dp {
		dp[i] = make([]bool, l)
	}
	for j := 1; j < l; j++ {
		for i := 0; i < j; i++ {
			if s[i] == s[j] {
				if j-i < 3 {
					dp[i][j] = true
				} else {
					dp[i][j] = dp[i+1][j-1]
				}
			} else {
				dp[i][j] = false
			}
			if dp[i][j] == true {
				if j-i+1 > maxLen {
					maxLen = j - i + 1
					start = i
				}
			}
		}
	}
	return s[start : start+maxLen]
}
