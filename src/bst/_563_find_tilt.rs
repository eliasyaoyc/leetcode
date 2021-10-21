use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, ret: &mut i32) -> i32 {
            if let Some(mut node) = node {
                let v = node.borrow().val;
                let lsum = dfs(node.borrow_mut().left.take(), ret);
                let rsum = dfs(node.borrow_mut().right.take(), ret);
                *ret += (lsum - rsum).abs();
                return lsum + rsum + v;
            } else {
                return 0;
            }
        }
        let mut ret = 0;
        dfs(root, &mut ret);
        ret
    }
}