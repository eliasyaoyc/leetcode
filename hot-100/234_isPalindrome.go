package hot_100

func isPalindrome(head *ListNode) bool {
	var array []int
	for head != nil {
		array = append(array, head.Val)
		head = head.Next
	}
	left, right := 0, len(array)-1
	for left < right {
		if array[left] != array[right] {
			return false
		}
		left++
		right--
	}
	return true
}
