use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}


impl Solution {
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            if node.borrow().val >= low && node.borrow().val <= high {
                node.borrow_mut().left = Self::trim_bst(left,low,high);
                node.borrow_mut().right = Self::trim_bst(right,low,high);
                return Some(node.clone());
            }else if node.borrow().val < low {
                return Self::trim_bst(right,low,high);
            }else if node.borrow().val > high {
                return Self::trim_bst(left,low,high);
            }
        }
        None
    }
}