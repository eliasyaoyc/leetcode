package leetcode_go

import "sort"

func pileBox(box [][]int) int {
	if len(box) == 1 {
		return box[0][2]
	}

	sort.Slice(box, func(i, j int) bool {
		return box[i][0] < box[j][0]
	})

	dp := make([]int, len(box))
	dp[0] = box[0][2]
	ret := box[0][2]

	for i := 1; i < len(box); i++ {
		dp[i] = box[i][2]

		for j := 0; j < i; j++ {
			if box[i][0] > box[j][0] && box[i][1] > box[j][1] && box[i][2] > box[j][2] {
				dp[i] = max(dp[i], dp[j]+box[i][2])
			}
		}
		ret = max(ret, dp[i])
	}
	return ret
}
