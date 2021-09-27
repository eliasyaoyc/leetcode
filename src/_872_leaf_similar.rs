use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn pre_order(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(node) = root.as_ref() {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    ans.push(node.borrow().val);
                }
                pre_order(node.borrow_mut().left.take(), ans);
                pre_order(node.borrow_mut().right.take(), ans);
            }
        }
        let mut root1_vec = vec![];
        let mut root2_vec = vec![];
        pre_order(root1, &mut root1_vec);
        pre_order(root2, &mut root2_vec);
        if root1_vec.is_empty() || root2_vec.is_empty() {
            return false;
        }
        root1_vec == root2_vec
    }
}