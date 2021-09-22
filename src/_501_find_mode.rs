use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn in_order(root: Option<Rc<RefCell<TreeNode>>>, cur: &mut i32, count: &mut i32, max: &mut i32, ans: &mut Vec<i32>) {
            if let Some(node) = root.as_ref() {
                in_order(node.borrow_mut().right.take(), cur, count, max, ans);
                let val = node.borrow().val;
                if val != *cur {
                    *count = 1;
                    *cur = val;
                }else {
                    *count += 1;
                }

                if *count == *max {
                    ans.push(val);
                }else if *count > *max {
                    *max = *count;
                    ans.clear();
                    ans.push(val);
                }

                in_order(node.borrow_mut().left.take(), cur, count, max, ans);
            }
        }
        let mut ret = vec![];
        let (mut cur, mut count, mut max) = (0, 1, 1);
        in_order(root, &mut cur, &mut count, &mut max, &mut ret);
        ret
    }
}