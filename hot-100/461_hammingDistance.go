package hot_100

import "math/bits"

func hammingDistance(x int, y int) int {
	return bits.OnesCount(uint(x ^ y))
}

func hammingDistance1(x int, y int) int {
	var ans int
	for i := x ^ y; i > 0; i >>= 1 {
		ans += i & 1
	}
	return ans
}
