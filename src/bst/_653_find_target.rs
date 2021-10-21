use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        fn in_order(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
            if let Some(node) = root.as_ref() {
                in_order(node.borrow_mut().left.take(), ret);
                ret.push(node.borrow().val);
                in_order(node.borrow_mut().right.take(), ret);
            }
        }
        let mut ret = vec![];
        in_order(root, &mut ret);
        for i in 0..ret.len() {
            for j in i + 1..ret.len() {
                if ret[i] + ret[j] == k {
                    return true;
                }
            }
        }
        false
    }
}