use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn in_order(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(node) = root.as_ref() {
                in_order(node.borrow_mut().left.take(), ans);
                ans.push(node.borrow().val);
                in_order(node.borrow_mut().right.take(), ans);
            }
        }
        let mut ans = vec![];
        let mut min = i32::MAX;
        in_order(root, &mut ans);
        let mut index = 0;
        while index < ans.len() - 1 {
            min = min.min(ans[index + 1] - ans[index]);
            index += 1;
        }
        min
    }
}