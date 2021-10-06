use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = vec![root.unwrap()];
        let mut level = 0;
        while !queue.is_empty() {
            let v = queue.iter().map(|node|node.borrow().val).collect::<Vec<i32>>();
            if v.iter().any(|&x| x % 2 == level % 2) {
                return false;
            }
            if v
                .windows(2)
                .any(|w| (level % 2 == 0 && w[0] >= w[1]) || (level % 2 == 1 && w[0] <= w[1]))
            {
                return false;
            }
            queue = queue.iter().fold(vec![], |mut acc, node| {
                if let Some(left) = node.borrow().left.as_ref() {
                    acc.push(left.clone());
                }
                if let Some(right) = node.borrow().right.as_ref() {
                    acc.push(right.clone());
                }
                acc
            });
            level += 1;
        }
        true
    }
}