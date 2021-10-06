use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, x: i32, left: &mut i32, right: &mut i32) -> i32 {
            if let Some(node) = root.as_ref() {
                let left_n = dfs(node.borrow_mut().left.take(), x, left, right);
                let right_n = dfs(node.borrow_mut().right.take(), x, left, right);
                if node.borrow().val == x {
                    *left = left_n;
                    *right = right_n;
                }
                left_n + right_n + 1
            } else { 0 }
        }
        let half = n / 2;
        let mut left = 0_i32;
        let mut right = 0_i32;
        dfs(root, x, &mut left, &mut right);
        if left > half || right > half || left + right < half {
            return true;
        }else {
            return false;
        }
    }
}