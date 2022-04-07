package hot_100

import "math"

func dailyTemperatures(temperatures []int) []int {
	l := len(temperatures)
	ans := make([]int, l)
	next := make([]int, 101)
	for i := 0; i < 101; i++ {
		next[i] = math.MaxInt32
	}
	for i := l - 1; i >= 0; i-- {
		warmerIndex := math.MaxInt32
		for t := temperatures[i] + 1; t <= 100; t++ {
			if next[t] < warmerIndex {
				warmerIndex = next[t]
			}
		}
		if warmerIndex < math.MaxInt32 {
			ans[i] = warmerIndex - i
		}
		next[temperatures[i]] = i
	}
	return ans
}

func dailyTemperatures1(temperatures []int) []int {
	length := len(temperatures)
	ans := make([]int, length)
	stack := []int{}
	for i, t := range temperatures {
		for len(stack) > 0 && t > temperatures[stack[len(stack)-1]] {
			preIndex := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			ans[preIndex] = i - preIndex
		}
		stack = append(stack, i)
	}
	return ans
}
