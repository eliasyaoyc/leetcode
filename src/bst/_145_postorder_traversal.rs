use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn posterorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn post_order(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(node) = root.as_ref() {
                post_order(node.borrow_mut().left.take(), ans);
                post_order(node.borrow_mut().right.take(), ans);
                ans.push(node.borrow().val);
            }
        }
        if root.is_none() {
            return vec![];
        }
        let mut ret = vec![];
        post_order(root, &mut ret);
        ret
    }
}