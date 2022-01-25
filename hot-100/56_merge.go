package hot_100

import "sort"

func merge(intervals [][]int) [][]int {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][0] < intervals[j][0]
	})
	var ans [][]int
	left, right := intervals[0][0], intervals[0][1]
	for i := 1; i < len(intervals); i++ {
		if right >= intervals[i][0] {
			right = max(right, intervals[i][1])
		} else {
			ans = append(ans, []int{left, right})
			left, right = intervals[i][0], intervals[i][1]
		}
	}
	ans = append(ans, []int{left, right})
	return ans
}
