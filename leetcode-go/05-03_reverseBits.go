package leetcode_go

func reverseBits(num int) int {
	if num == 0 {
		return 1
	}
	max, current, reserver := 0, 0, 0
	for i := 0; i < 32; i++ {
		if num&1 == 1 {
			current++
			reserver++
		} else {
			reserver = current + 1
			current = 0
		}
		if max < reserver {
			max = reserver
		}
		num = num >> 1
	}
	return max
}
