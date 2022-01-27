package hot_100

func mergeKLists(lists []*ListNode) *ListNode {
	var merge func(l1 *ListNode, l2 *ListNode) *ListNode
	merge = func(l1 *ListNode, l2 *ListNode) *ListNode {
		dummy := &ListNode{}
		cur := dummy
		for l1 != nil || l2 != nil {
			if l1 != nil && l2 != nil {
				if l1.Val < l2.Val {
					cur.Next = l1
					l1 = l1.Next
				} else {
					cur.Next = l2
					l2 = l2.Next
				}
				cur = cur.Next
			} else if l1 != nil {
				cur.Next = l1
			} else if l2 != nil {
				cur.Next = l2
			}
		}
		return dummy.Next
	}
	var prev, cur *ListNode
	for i := 0; i < len(lists); i++ {
		if i == 0 {
			prev = lists[i]
			continue
		}
		cur = lists[i]
		prev = merge(prev, cur)
	}
	return prev
}
