use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, parent_maps: &mut HashMap<i32, Vec<i32>>) {
            if let Some(node) = root.as_ref() {
                let val = node.borrow().val;
                if let Some(left) = node.borrow_mut().left.take() {
                    parent_maps.entry(val).or_default().push(left.borrow().val);
                    parent_maps.entry(left.borrow().val).or_default().push(val);
                    dfs(Some(left), parent_maps);
                }
                if let Some(right) = node.borrow_mut().right.take() {
                    parent_maps.entry(val).or_default().push(right.borrow().val);
                    parent_maps.entry(right.borrow().val).or_default().push(val);
                    dfs(Some(right), parent_maps);
                }
            }
        }
        if target.is_none() || root.is_none() {
            return vec![];
        }
        let mut ret = vec![];
        let parent_maps: &mut HashMap<i32, Vec<i32>> = &mut HashMap::new();
        dfs(root, parent_maps);

        let mut visited = HashSet::<i32>::new();

        let start = target.unwrap().borrow().val;

        visited.insert(start);

        let mut queue = VecDeque::<(i32, i32)>::new();
        queue.push_back((start, 0));

        while let Some((val, count)) = queue.pop_front() {
            if count == k {
                ret.push(val);
            } else {
                for &mut v in parent_maps.entry(val).or_default() {
                    if visited.insert(v) {
                        queue.push_back((v, count + 1));
                    }
                }
            }
        }
        ret
    }
}