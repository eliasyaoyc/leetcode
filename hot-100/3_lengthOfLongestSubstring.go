package hot_100

func lengthOfLongestSubstring(s string) int {
	m, left, ans := map[byte]int{}, 0, 0
	for i := 0; i < len(s); i++ {
		if v, exist := m[s[i]]; exist {
			left = max(left, v+1) // 向右移动
		}
		m[s[i]] = i
		ans = max(ans, i-left+1)
	}
	return ans
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
