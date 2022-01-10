package leetcode_go

import "sort"

func findMinArrowShots(points [][]int) int {
	sort.Slice(points, func(i, j int) bool {
		return points[i][0] < points[j][0]
	})
	ret := 1
	for i := 1; i < len(points); i++ {
		if points[i][0] > points[i-1][1] {
			ret++
		} else {
			// 1,6   2,8  7,12  10,16 处理这种情况
			points[i][1] = min(points[i][1], points[i-1][1])
		}
	}
	return ret
}
