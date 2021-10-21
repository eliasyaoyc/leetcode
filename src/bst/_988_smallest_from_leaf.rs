use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn pre_order(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<char>, res: &mut String) {
            if let Some(node) = root.as_ref() {
                let val = (node.borrow().val as u8 + b'a') as char;
                ans.push(val);
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    let s: String = ans.iter().rev().copied().collect();
                    if res.is_empty() || s < *res {
                        *res = s;
                    }
                }
                pre_order(node.borrow_mut().left.take(), ans, res);
                pre_order(node.borrow_mut().right.take(), ans, res);
                ans.pop();
            }
        }
        let mut ans = vec![];
        let mut res = String::new();
        pre_order(root, &mut ans, &mut res);
        res
    }
}