package hot_100

func isMatch(s string, p string) bool {
	if len(s) == 0 {
		if len(p) == 0 {
			return true
		} else {
			return false
		}
	}
	m := len(s) + 1
	n := len(p) + 1
	dp := make([][]bool, m)
	for i := range dp {
		dp[i] = make([]bool, n)
	}
	dp[0][0] = true
	for i := 2; i < n; i++ {
		if p[i-1] == '*' {
			dp[0][i] = dp[0][i-2]
		}
	}

	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			if s[i-1] == p[j-1] || p[j-1] == '.' {
				dp[i][j] = dp[i-1][j-1]
			} else if p[j-1] == '*' {
				if p[j-2] == s[i-1] || p[j-2] == '.' {
					dp[i][j] = dp[i-1][j] || dp[i][j-2]
				} else {
					dp[i][j] = dp[i][j-2]
				}
			} else {
				dp[i][j] = false
			}
		}
	}
	return dp[m-1][n-1]
}
