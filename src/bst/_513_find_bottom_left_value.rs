use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn bfs(nodes: Vec<Rc<RefCell<TreeNode>>>, val: &mut i32) {
            if nodes.is_empty() {
                return;
            }
            *val = nodes.first().unwrap().borrow().val;
            bfs(nodes.iter().fold(vec![], |mut acc, node| {
                if let Some(ref left) = node.borrow().left {
                    acc.push(left.clone());
                }
                if let Some(ref right) = node.borrow().right {
                    acc.push(right.clone());
                }
                acc
            }), val);
        }

        let mut ret = 0_i32;
        if root.is_none() {
            return ret;
        }
        bfs(vec![root.unwrap()], &mut ret);
        ret
    }
}