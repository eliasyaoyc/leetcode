package hot_100

func subsetts(nums []int) [][]int {
	var ans [][]int
	var backtrack func(cur []int, index int)
	backtrack = func(cur []int, index int) {
		tmp := make([]int, len(cur))
		copy(tmp, cur)
		ans = append(ans, tmp)
		for i := index; i < len(nums); i++ {
			cur = append(cur, nums[i])
			backtrack(cur, i+1)
			cur = cur[:len(cur)-1]
		}
	}
	backtrack([]int{}, 0)
	return ans
}
