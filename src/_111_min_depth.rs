use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;
use std::mem::take;

struct Solution {}

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none(){
            return 0;
        }

        let left = root.clone().unwrap().borrow_mut().left.take();
        let right = root.unwrap().borrow_mut().right.take();
        if left.is_none() && right.is_none() {
            return 1;
        }
        if left.is_none() {
            return Self::min_depth(right) + 1;
        }
        if right.is_none() {
            return Self::min_depth(left) + 1;
        }
        Self::min_depth(left).min(Self::min_depth(right)) + 1
    }
}