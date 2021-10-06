use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if head.is_none() || root.is_none() {
            return true;
        }
        let mut head = head;
        let mut arr = vec![];
        while let Some(node) = head.as_ref() {
            arr.push(node.val);
            head = node.next.clone();
        }

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, arr: &mut [i32], ret: &mut bool) {
            if let Some(node) = root.as_ref() {
                path.push(node.borrow().val);
                if path.ends_with(arr) {
                    *ret = true;
                }
                dfs(node.borrow_mut().left.take(),path,arr,ret);
                dfs(node.borrow_mut().right.take(),path,arr,ret);
                path.pop();
            }
        }
        let mut ret = false;
        dfs(root, &mut vec![], &mut arr,&mut ret);
        ret
    }
}