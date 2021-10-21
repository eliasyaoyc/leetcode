use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut max: i32, ans: &mut i32) {
            if let Some(node) = root.as_ref() {
                if node.borrow().val >= max {
                    *ans += 1;
                    max = node.borrow().val;
                }
                dfs(node.borrow_mut().left.take(),max,ans);
                dfs(node.borrow_mut().right.take(),max,ans);
            }
        }
        let mut ans = 0_i32;
        dfs(root, i32::MIN, &mut ans);
        ans
    }
}