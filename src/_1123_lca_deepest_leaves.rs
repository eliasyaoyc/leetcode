use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;
use std::cmp::Ordering;

struct Solution{}

impl Solution {
    pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32,Option<Rc<RefCell<TreeNode>>>){
            if let Some(node) = root.as_ref(){
                let (h_l, l) = dfs(&node.borrow().left);
                let (h_r, r) = dfs(&node.borrow().right);
                match h_l.cmp(&h_r) {
                    Ordering::Less => (h_r + 1, r),
                    Ordering::Greater => (h_l + 1, l),
                    Ordering::Equal => (h_l +1 , Some(node.clone()))
                }
            }else {
                (0,None)
            }
        }
        dfs(&root).1
    }
}