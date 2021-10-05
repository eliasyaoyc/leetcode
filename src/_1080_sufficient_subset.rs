use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn sufficient_subset(root: Option<Rc<RefCell<TreeNode>>>, limit: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, limit: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root.as_ref() {
                let v = node.borrow().val;

                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    if v >= limit {
                        Some(node.clone())
                    } else {
                        None
                    }
                } else {
                    let left = dfs(node.borrow_mut().left.take(), limit - v);
                    let right = dfs(node.borrow_mut().right.take(), limit - v);
                    if left.is_some() || right.is_some() {
                        node.borrow_mut().left = left;
                        node.borrow_mut().right = right;
                        Some(node.clone())
                    } else {
                        None
                    }
                }
            } else {
                None
            }
        }
        dfs(root,limit)
    }
}