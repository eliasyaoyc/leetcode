use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn add_one_row(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })));
        }
        fn dfs(root: &mut Option<Rc<RefCell<TreeNode>>>, dep: i32, d: i32, v: i32) {
            if let Some(mut node) = root.as_ref() {
                if dep == d {
                    let new_l = Some(Rc::new(RefCell::new(TreeNode{
                        val: v,
                        left: node.borrow_mut().left.take(),
                        right: None,
                    })));
                    let new_r = Some(Rc::new(RefCell::new(TreeNode{
                        val: v,
                        left: None,
                        right: node.borrow_mut().right.take(),
                    })));
                    node.borrow_mut().left = new_l;
                    node.borrow_mut().right = new_r;
                }
                dfs(&mut node.borrow_mut().left, dep + 1, d, v);
                dfs(&mut node.borrow_mut().right, dep + 1, d, v);
            }
        }
        dfs(&mut root, 1, depth - 1, val);
        root
    }
}

#[test]
fn test() {}