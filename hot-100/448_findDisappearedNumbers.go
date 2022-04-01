package hot_100

// 暴力求解
func findDisappearedNumbers(nums []int) []int {
	l := len(nums)
	var ans []int
	for i := 1; i <= l; i++ {
		exist := false
		for _, n := range nums {
			if i == n {
				exist = true
				break
			}
		}
		if !exist {
			ans = append(ans, i)
		}
	}
	return ans
}

func findDisappearedNumbers1(nums []int) []int {
	var ans []int
	l := len(nums)
	for _, v := range nums {
		v = (v - 1) % l
		nums[v] += l
	}
	for i, v := range nums {
		if v <= l {
			ans = append(ans, i+1)
		}
	}
	return ans
}
