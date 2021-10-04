use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>, to_delete: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root.as_ref() {
                let left = dfs(node.borrow_mut().left.take(), ans, to_delete);
                let right = dfs(node.borrow_mut().right.take(), ans, to_delete);

                if to_delete.contains(&node.borrow().val) {
                    if left.is_some() {
                        ans.push(left.clone());
                    }
                    if right.is_some() {
                        ans.push(right.clone());
                    }
                    None
                } else {
                    node.borrow_mut().left = left.clone();
                    node.borrow_mut().right = right.clone();
                    Some(node.clone())
                }
            } else { None }
        }
        let mut ans = vec![];
        let root = dfs(root, &mut ans, &to_delete);
        if root.is_some() {
            ans.push(root);
        }
        ans
    }
}