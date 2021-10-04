use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut index = 1;
        let mut min = (1, i32::MIN);
        let mut queue = vec![root.unwrap()];
        while !queue.is_empty() {
            let v = queue.iter().fold(0, |mut acc, node| {
                acc += node.borrow().val;
                acc
            });
            if v > min.1 {
                min = (index, v);
            }
            queue = queue.iter().fold(vec![], |mut acc, node| {
                if let Some(left) = &node.borrow().left {
                    acc.push(left.clone())
                }
                if let Some(right) = &node.borrow().right {
                    acc.push(right.clone())
                }
                acc
            });
            index += 1;
        }
        min.0
    }
}