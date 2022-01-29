package hot_100

// ⚠️ 要求使用 On 时间复杂度，如果先排序在循环进行判断，则时间复杂度为 On2
func longestConsecutive(nums []int) int {
	numSet := map[int]bool{}
	for _, num := range nums {
		numSet[num] = true
	}
	ans := 0
	for v := range numSet {
		if !numSet[v-1] {
			curNum := v
			curLen := 1
			for numSet[curNum+1] {
				curNum++
				curLen++
			}
			if curLen > ans {
				ans = curLen
			}
		}
	}
	return ans
}
