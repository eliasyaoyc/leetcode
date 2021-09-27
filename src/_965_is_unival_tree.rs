use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn pre_order(root: Option<Rc<RefCell<TreeNode>>>,mut val: i32) -> bool{
            if let Some(node) = root.as_ref(){
                if val == -1 {
                    val = node.borrow().val;
                }else {
                    if val != node.borrow().val{
                        return false;
                    }
                }
                let left = pre_order(node.borrow_mut().left.take(),val);
                let right = pre_order(node.borrow_mut().right.take(),val);
                return left && right;
            }
            true
        }
        pre_order(root,-1)
    }
}