use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut parent: Option<(i32, i32)>, ans: &mut i32) {
            if let Some(node) = root.as_ref() {
                let v = node.borrow().val;
                parent = if let Some((min, max)) = parent {
                    *ans = (*ans).max(((min - v).abs()).max((max - v).abs()));
                    Some((min.min(v),max.max(v)))
                } else {
                    Some((v, v))
                };
                dfs(node.borrow_mut().left.take(),parent,ans);
                dfs(node.borrow_mut().right.take(),parent,ans);
            }
        }
        let mut max = 0_i32;
        dfs(root, None, &mut max);
        max
    }
}