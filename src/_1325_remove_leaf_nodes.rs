use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn post_order(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root.as_ref() {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                node.borrow_mut().left = post_order(left, target);
                node.borrow_mut().right = post_order(right, target);
                if node.borrow().val == target && node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return None;
                }
                return Some(node.clone());
            }
            None
        }
        post_order(root,target)
    }
}