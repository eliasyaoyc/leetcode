use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        fn pre_oder(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32, low: i32, high: i32) {
            if let Some(node) = root.as_ref() {
                if node.borrow().val >= low && node.borrow().val <= high {
                    *ans += node.borrow().val;
                }
                pre_oder(node.borrow_mut().left.take(), ans, low, high);
                pre_oder(node.borrow_mut().right.take(), ans, low, high);
            }
        }
        let mut ans = 0_i32;
        pre_oder(root, &mut ans, low, high);
        ans
    }
}