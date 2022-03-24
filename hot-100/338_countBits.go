package hot_100

// brain kernighan 算法
func countBits(n int) []int {
	var bk func(n int) int
	bk = func(n int) (ans int) {
		for ; n > 0; n &= n - 1 {
			ans++
		}
		return
	}
	var ans []int
	for i := 0; i < n; i++ {
		ans[i] = bk(i)
	}
	return ans
}

func countBits1(n int) []int {
	ans := make([]int, n+1)
	for i := 1; i < n+1; i++ {
		ans[i] = ans[i&(i-1)] + 1
	}
	return ans
}
