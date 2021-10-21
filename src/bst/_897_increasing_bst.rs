use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn in_order(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(node) = root.as_ref() {
                in_order(node.borrow_mut().left.take(), ans);
                ans.push(node.borrow().val);
                in_order(node.borrow_mut().right.take(), ans);
            }
        }
        let mut ans = vec![];
        in_order(root, &mut ans);
        let mut curr = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let mut head = curr.clone();
        for i in ans {
            curr.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(i))));
            curr = curr.clone().unwrap().borrow().right.clone();
        }
        head = head.clone().unwrap().borrow().right.clone();
        head
    }
}