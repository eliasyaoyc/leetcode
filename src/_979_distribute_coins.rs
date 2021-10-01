use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> i32 {
            if let Some(node) = root.as_ref() {
                let left = dfs(node.borrow_mut().left.take(), count);
                let right = dfs(node.borrow_mut().right.take(), count);
                *count += i32::abs(left) + i32::abs(right);
                node.borrow().val + left + right - 1
            } else {
                0
            }
        }
        let mut count = 0_i32;
        dfs(root, &mut count);
        count
    }
}