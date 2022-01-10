package leetcode_go

import "sort"

func eraseOverlapIntervals(intervals [][]int) int {
	dp := make([]int, len(intervals))
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][0] < intervals[j][0]
	})
	for i := range dp {
		dp[i] = 1
	}
	for i := 1; i < len(intervals); i++ {
		for j := 0; j < i; j++ {
			if intervals[j][1] <= intervals[i][0] {
				dp[i] = max(dp[i], dp[j]+1)
			}
		}
	}
	return len(intervals) - max1(dp...)
}

func max1(a ...int) int {
	res := a[0]
	for _, v := range a {
		if v > res {
			res = v
		}
	}
	return res
}
