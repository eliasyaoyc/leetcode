use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn insert_into_max_tree(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            if node.borrow().val < val {
                let mut new_node = TreeNode::new(val);
                new_node.left = Some(node.clone());
                Some(Rc::new(RefCell::new(new_node)))
            } else {
                let right = node.borrow_mut().right.take();
                node.borrow_mut().right = Self::insert_into_max_tree(right, val);
                Some(node.clone())
            }
        } else {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}