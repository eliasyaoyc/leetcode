use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;
use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            match (root.borrow().left.clone(), root.borrow().right.clone()) {
                (None, None) | (_, None) | (None, _) => -1,
                (Some(left), Some(right)) => {
                    let (mut l, mut r) = (left.borrow().val, right.borrow().val);
                    if (root.borrow().val == l) {
                        l = Solution::find_second_minimum_value(Some(left))
                    }
                    if (root.borrow().val == r) {
                        r = Solution::find_second_minimum_value(Some(right))
                    }
                    if l != -1 && r != -1 {
                        return l.min(r);
                    }
                    if l != -1 {
                        return l;
                    } else {
                        return r;
                    }
                }
            }
        }else { -1 }
    }
}

#[test]
fn t() {
    let mut root = TreeNode::new(2);
    let mut root_right = TreeNode::new(2);

    root.left = Some(Rc::new(RefCell::new(root_right)));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    let result = Solution::find_second_minimum_value(Some(Rc::new(RefCell::new(root))));
    println!("{}", result);
}