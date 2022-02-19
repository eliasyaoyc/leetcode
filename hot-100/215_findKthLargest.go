package hot_100

func findKthLargest(nums []int, k int) int {
	target := len(nums) - k
	left, right := 0, len(nums)-1
	var partition func(left, right int) int
	partition = func(left, right int) int {
		prev := nums[left]
		j := left
		for i := left + 1; i <= right; i++ {
			if nums[i] < prev {
				j++
				swap(nums, j, i)
			}
		}
		swap(nums, j, left)
		return j
	}
	for {
		index := partition(left, right)
		if index == target {
			return nums[index]
		} else if index < target {
			left = index + 1
		} else {
			right = index - 1
		}
	}
}

func swap(nums []int, index1, index2 int) {
	temp := nums[index1]
	nums[index1] = nums[index2]
	nums[index2] = temp
}
