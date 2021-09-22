use std::rc::Rc;
use std::cell::RefCell;

mod _102_level_order;
mod _103_zigzag_level_order;
mod _107_level_order_bottom;
mod _111_min_depth;
mod _113_path_sum;
mod _129_sum_numbers;
mod _145_postorder_traversal;
mod _331_is_valid_serialization;
mod _404_sum_of_left_levels;
mod todo_rob_337;
mod _449_codec;

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