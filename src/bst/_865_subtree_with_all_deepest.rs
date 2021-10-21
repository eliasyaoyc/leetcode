use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
            if let Some(node) = root.as_ref() {
                let (left, lCount) = dfs(&node.borrow().left);
                let (right, rCount) = dfs(&node.borrow().right);
                match lCount.cmp(&rCount) {
                    std::cmp::Ordering::Equal => (Some(node.clone()), lCount + 1),
                    std::cmp::Ordering::Greater => (left, lCount + 1),
                    std::cmp::Ordering::Less => (right, rCount + 1),
                }
            } else {
                (None, 0)
            }
        }
        dfs(&root).0
    }
}