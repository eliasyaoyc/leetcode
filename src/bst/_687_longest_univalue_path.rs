use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn post_order(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32, val: i32) -> i32 {
            if let Some(node) = root.as_ref() {
                let v = node.borrow().val;
                let left = post_order(node.borrow_mut().left.take(), max, v);
                let right = post_order(node.borrow_mut().right.take(), max, v);
                *max = i32::max(left + right, *max);
                if v == val {
                    i32::max(left, right) + 1
                } else { 0 }
            }else { 0 }
        }
        let mut max = 0_i32;
        post_order(root, &mut max, 0);
        max
    }
}