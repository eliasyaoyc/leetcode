use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn pre_order(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut String) {
            if let Some(node) = root.as_ref() {
                ret.push_str(&node.borrow().val.to_string());
                if node.borrow().left.is_none() && node.borrow().right.is_some() {
                    ret.push_str("(");
                    ret.push_str(")");
                }

                if node.borrow().left.is_some() {
                    ret.push_str("(");
                    pre_order(node.borrow_mut().left.take(), ret);
                    ret.push_str(")");
                }

                if node.borrow().right.is_some() {
                    ret.push_str("(");
                    pre_order(node.borrow_mut().right.take(), ret);
                    ret.push_str(")");
                }
            }
        }
        let mut ret = String::new();
        pre_order(root, &mut ret);
        ret
    }
}

#[test]
fn test() {
    let mut root = TreeNode::new(1);
    let mut root_right = TreeNode::new(2);
    root_right.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    root.left = Some(Rc::new(RefCell::new(root_right)));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    let result = Solution::tree2str(Some(Rc::new(RefCell::new(root))));
    println!("{}", result);
}