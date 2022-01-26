package hot_100

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	ans := &ListNode{}
	ans.Next = head
	var pre *ListNode
	cur := ans
	i := 1
	for head != nil {
		if i >= n {
			pre = cur
			cur = cur.Next
		}
		head = head.Next
		i++
	}
	pre.Next = pre.Next.Next
	return ans.Next
}
