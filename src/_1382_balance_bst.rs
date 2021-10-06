use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(node) = root.as_ref() {
                dfs(node.borrow_mut().left.take(), ans);
                ans.push(node.borrow().val);
                dfs(node.borrow_mut().right.take(), ans);
            }
        }
        fn construct(ans: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if ans.is_empty() {
                return None;
            }
            let mid = ans.len() / 2 as usize;
            let mut node = TreeNode::new(ans[mid]);
            let left = construct(&ans[..mid]);
            let right = construct(&ans[mid + 1..]);
            node.left = left;
            node.right = right;
            Some(Rc::new(RefCell::new(node)))
        }
        let mut ans = vec![];
        dfs(root, &mut ans);
        construct(&mut ans)
    }
}