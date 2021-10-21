use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, even: bool, ans: &mut i32) {
            if let Some(node) = root.as_ref() {
                let left  = node.borrow_mut().left.take();
                let right  = node.borrow_mut().right.take();
                if even {
                    if left.is_some(){
                        *ans += left.clone().unwrap().borrow().val;
                    }
                    if right.is_some() {
                        *ans += right.clone().unwrap().borrow().val;
                    }
                }
                if node.borrow().val % 2 == 0 {
                    dfs(left,true,ans);
                    dfs(right,true,ans);
                }else {
                    dfs(left,false,ans);
                    dfs(right,false,ans);
                }
            }
        }
        let mut ans = 0_i32;
        dfs(root,false,&mut ans);
        ans
    }
}