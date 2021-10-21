use std::cell::{RefCell, Ref};
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn prune_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn prune(root: Option<&mut Rc<RefCell<TreeNode>>>) -> bool {
            if let Some(node) = root.as_ref() {
                let left = prune(node.borrow_mut().left.as_mut());
                let right = prune(node.borrow_mut().right.as_mut());
                if !left {
                    node.borrow_mut().left.take();
                }
                if !right {
                    node.borrow_mut().right.take();
                }
                return node.borrow().val == 1 || left || right;
            }
            false
        }
        if !prune(root.as_mut()) {
            return None;
        }
        root
    }
}

#[test]
fn t() {
    let mut root = TreeNode::new(1);
    let mut root_right = TreeNode::new(0);
    root_right.left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    root_right.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    root.right = Some(Rc::new(RefCell::new(root_right)));

    let result = Solution::prune_tree(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", result);
}