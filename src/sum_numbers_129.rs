use std::cell::RefCell;
use crate::TreeNode;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cur: i32, ans: &mut i32) {
            if let Some(mut node) = root.as_ref(){
                let mut node = node.borrow_mut();

                let cur = cur * 10 + node.val;
                match (node.left.take(), node.right.take()) {
                    (None,None) => {
                        *ans += cur;
                    }
                    (Some(left),None) => {
                        dfs(Some(left),cur,ans);
                    }
                    (None,Some(right)) => {
                        dfs(Some(right),cur,ans);
                    }
                    (Some(left),Some(right)) => {
                        dfs(Some(left),cur,ans);
                        dfs(Some(right),cur,ans);
                    }
                }
            }
        }

        if root.is_none() {
            return 0;
        }

        let ans = &mut 0_i32;
        dfs(root, 0, ans);
        *ans
    }
}