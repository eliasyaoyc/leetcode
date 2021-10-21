use std::cell::RefCell;
use crate::TreeNode;
use std::rc::Rc;
use core::slice::SplitInclusiveMut;

struct Solution {}

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn order(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(node) = root.as_ref() {
                if let Some(n) = &node.borrow().left{
                    if n.borrow().left.is_none() && n.borrow().right.is_none() {
                        *sum += n.borrow().val;
                    }
                }
                order(node.borrow_mut().left.take(), sum);
                order(node.borrow_mut().right.take(),sum);
            }
        }
        if root.is_none() {
            return 0;
        }
        let mut sum = 0;
        order(root, &mut sum);
        sum
    }
}