use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>) -> i32 {
            if let Some(node) = root.as_ref() {
                let mut val = node.borrow().val;
                val += dfs(node.borrow_mut().left.take(), map);
                val += dfs(node.borrow_mut().right.take(), map);
                let count = map.entry(val).or_insert(0);
                *count += 1;
                val
            } else {
                0
            }
        }

        let mut ret = HashMap::new();
        dfs(root, &mut ret);
        let mut unSort: Vec<_> = ret.iter().collect();
        unSort.sort_by(|a, b| b.1.cmp(a.1));
        if unSort.len() < 1 {
            return vec![];
        }
        let max = unSort[0].1;
        unSort.iter().filter(|e| e.1 == max).map(|e| *e.0).collect::<Vec<i32>>()
    }
}