use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, parent_val: i32, sum: &mut i32) {
            if let Some(node) = root.as_ref() {
                let val = node.borrow().val + parent_val * 2;
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    *sum += val;
                }else {
                    dfs(&node.borrow().left,val,sum);
                    dfs(&node.borrow().right,val,sum);
                }
            }
        }
        let mut sum = 0_i32;
        dfs(&root,0,&mut sum);
        sum
    }
}