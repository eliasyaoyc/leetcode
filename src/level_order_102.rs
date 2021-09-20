use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn order(mut ans: Vec<Vec<i32>>, nodes: Vec<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            order(if nodes.is_empty()
                  {
                      return ans;
                  } else {
                ans.push(nodes.iter().map(|node| node.borrow().val).collect());
                ans
            },
                  nodes.iter().fold(vec![], |mut acc, node| {
                      if let Some(ref left) = node.borrow().left {
                          acc.push(left.clone());
                      }
                      if let Some(ref right) = node.borrow().right {
                          acc.push(right.clone())
                      }
                      acc
                  }),
            )
        }

        if root.is_none() {
            return vec![];
        }

        order(vec![], vec![root.unwrap()])
    }
}