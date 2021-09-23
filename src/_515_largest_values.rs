use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn bfs(ans: Vec<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
            if !ans.is_empty() {
                ret.push(ans.iter().max_by_key(|x| x.borrow().val).unwrap().borrow().val);

                bfs(ans.iter().fold(vec![], |mut acc, node| {
                    if let Some(ref left) = node.borrow().left {
                        acc.push(left.clone());
                    }
                    if let Some(ref right) = node.borrow().right {
                        acc.push(right.clone());
                    }
                    acc
                }), ret);
            }
        }
        let mut ret = vec![];

        if root.is_none() {
            return ret;
        }

        bfs(vec![root.unwrap()], &mut ret);
        ret
    }
}