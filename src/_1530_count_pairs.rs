use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32, sum: &mut i32) -> Vec<i32> {
            if let Some(node) = root.as_ref() {
                let mut ret = vec![];
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    ret.push(0);
                    return ret;
                }

                let left = dfs(node.borrow_mut().left.take(), distance, sum);
                let right = dfs(node.borrow_mut().right.take(), distance, sum);

                for dis1 in &left {
                    for dis2 in &right {
                        if dis1 + dis2 + 2 <= distance {
                            *sum += 1;
                        }
                    }
                }

                for dis1 in left {
                    if dis1 + 1 < distance {
                        ret.push(dis1 + 1);
                    }
                }

                for dis2 in right {
                    if dis2 + 1 < distance {
                        ret.push(dis2 + 1);
                    }
                }
                ret
            } else {
                vec![]
            }
        }
        let mut ans = 0_i32;
        dfs(root,distance,&mut ans);
        ans
    }
}
