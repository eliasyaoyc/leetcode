use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = vec![root.unwrap()];
        let mut ret = 0_i32;
        while !queue.is_empty() {
            ret = queue.iter().fold(0, |mut acc, node| {
                acc += node.borrow().val;
                acc
            });
            queue = queue.iter().fold(vec![],|mut acc,node|{
                if let Some(left) = node.borrow().left.as_ref(){
                    acc.push(left.clone());
                }
                if let Some(right) = node.borrow().right.as_ref(){
                    acc.push(right.clone());
                }
                acc
            });
        }
        ret
    }
}