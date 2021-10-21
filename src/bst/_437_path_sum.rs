use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn prefix_order(root: Option<Rc<RefCell<TreeNode>>>, prefix_map: &mut HashMap<i32, i32>, target_sum: i32, prefix: i32) -> i32 {
            if let Some(node) = root.as_ref() {
                let prefix = prefix + node.borrow().val;
                let mut res = 0_i32;
                let count = *prefix_map.get(&(prefix - target_sum)).unwrap_or(&0);
                res += count;
                *prefix_map.entry(prefix).or_default() += 1;
                res += prefix_order(node.borrow_mut().left.take(), prefix_map, target_sum, prefix);
                res += prefix_order(node.borrow_mut().right.take(), prefix_map, target_sum, prefix);
                *prefix_map.entry(prefix).or_default() -= 1;
                res
            } else {
                0
            }
        }
        let prefix_map: &mut HashMap<i32, i32> = &mut HashMap::new();
        prefix_map.insert(0, 1);
        prefix_order(root, prefix_map, target_sum, 0)
    }
}