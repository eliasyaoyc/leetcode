package hot_100

import "sort"

var ans [][]int

func combinationSum(candidates []int, target int) [][]int {
	ans = [][]int{}
	sort.Ints(candidates)
	var dfs func(cur []int, index int)
	dfs = func(cur []int, index int) {
		sum := 0
		for i := range cur {
			sum += cur[i]
		}
		if sum == target {
			tmp := make([]int, len(cur))
			copy(tmp, cur)
			ans = append(ans, tmp)
			return
		}
		for i := index; i < len(candidates); i++ {
			if sum+candidates[i] > target {
				break
			}
			cur = append(cur, candidates[i])
			dfs(cur, i)
			cur = cur[:len(cur)-1]
		}
	}
	dfs([]int{}, 0)
	return ans
}
