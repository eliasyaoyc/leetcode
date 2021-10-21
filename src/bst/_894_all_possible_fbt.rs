struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n % 2 == 0 {
            return vec![];
        }
        if n == 1 {
            vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))]
        } else {
            let mut res = vec![];
            let mut l = 1;
            let mut r = n - 1 - l;
            while r > 0 {
                let left_trees = Self::all_possible_fbt(l);
                let right_trees = Self::all_possible_fbt(r);
                for left in &left_trees {
                    for right in &right_trees {
                        res.push(Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: left.clone(),
                            right: right.clone(),
                        }))))
                    }
                }
                r -= 2;
                l += 2;
            }
            res
        }
    }
}

#[test]
fn test(){
    let root = Solution::all_possible_fbt(7);
    println!("{:?}",root);
}