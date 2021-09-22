use std::cell::RefCell;
use crate::TreeNode;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn find(node: Rc<RefCell<TreeNode>>, target_sum: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            let mut node = node.borrow_mut();
            let target_sum = target_sum - node.val;
            path.push(node.val);
            match (node.left.take(), node.right.take()) {
                (None, None) => { if target_sum == 0 { res.push(path.clone()); } }
                (Some(left), None) => { find(left, target_sum, path, res); }
                (None, Some(right)) => { find(right, target_sum, path, res); }
                (Some(left), Some(right)) => {
                    find(left, target_sum, path, res);
                    find(right, target_sum, path, res);
                }
            }
            path.pop();
        }

        let mut res = Vec::new();
        if let Some(root) = root {
            find(root, target_sum, &mut Vec::new(), &mut res);
        };
        res
    }
}