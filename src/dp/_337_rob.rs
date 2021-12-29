use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn rob_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            if let Some(node) = root.as_ref() {
                let left = rob_tree(node.borrow_mut().left.take());
                let right = rob_tree(node.borrow_mut().right.take());
                let cur = node.borrow().val + left[0] + right[0];
                let not_cur = left[0].max(left[1]) + right[0].max(right[1]);
                return vec![not_cur, cur];
            }
            vec![]
        }
        let res = rob_tree(root);
        res[0].max(res[1])
    }
}