package hot_100

import "math"

// 要求通过 On 的时间复杂度来解决
func minWindow(s string, t string) string {
	if len(s) == 0 || len(t) == 0 {
		return ""
	}
	need := make([]int, 128)
	for _, v := range t {
		need[v-'A']++
	}
	left, right, size, count, start := 0, 0, math.MaxInt32, len(t), 0
	for right < len(s) {
		c := s[right]
		if need[c-'A'] > 0 {
			count--
		}
		need[c-'A']--
		if count == 0 {
			for left < right && need[s[left]-'A'] < 0 {
				need[s[left]-'A']++
				left++
			}
			if right-left+1 < size {
				size = right - left + 1
				start = left
			}
			need[s[left]-'A']++
			count++
			left++
		}
		right++
	}
	if size == math.MaxInt32 {
		return ""
	}
	return s[start : start+size]
}
