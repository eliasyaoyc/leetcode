package hot_100

func hasCycle(head *ListNode) bool {
	hashSet := map[*ListNode]struct{}{}
	for head != nil {
		if _, ok := hashSet[head]; ok {
			return true
		}
		hashSet[head] = struct{}{}
		head = head.Next
	}
	return false
}

func hasCycle1(head *ListNode) bool {
	if head == nil || head.Next == nil {
		return false
	}
	slow, fast := head, head.Next
	for fast != slow {
		if fast == nil || fast.Next == nil {
			return false
		}
		slow = slow.Next
		fast = fast.Next.Next
	}
	return true
}
