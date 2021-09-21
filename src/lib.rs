use std::rc::Rc;
use std::cell::RefCell;

mod level_order_102;
mod zigzag_level_order_103;
mod level_order_bottom_107;
mod min_depth_111;
mod path_sum_113;
mod sum_numbers_129;

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