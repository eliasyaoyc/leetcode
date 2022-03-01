package hot_100

/*
              1,  2,  3,  4
第一个数字 1    1  x   x   x
第二个数字 2    x  2   x   x
第三个数字 3    x  x   3   x
第四个数字 4    x  x   x   4
*/
func productExceptSelf(nums []int) []int {
	ans := make([]int, len(nums))
	head, tail := 1, 1
	for i := 0; i < len(nums); i++ {
		ans[i] = head
		head *= nums[i]
	}
	for i := len(nums) - 1; i > 0; i-- {
		tail *= nums[i]
		ans[i-1] *= tail
	}
	return ans
}
