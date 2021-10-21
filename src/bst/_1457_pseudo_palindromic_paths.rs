use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::TreeNode;

struct Solution {}

// 回文串的定义是 最多只有一个数字出现奇数
impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut path: u32, ans: &mut i32) {
            if let Some(node) = root.as_ref() {
                path ^= 1 << node.borrow().val; // ^ 相同的则为0，反之为1
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    if path.count_ones() < 2 {
                        *ans += 1;
                    }
                }
                dfs(node.borrow_mut().left.take(),path,ans);
                dfs(node.borrow_mut().right.take(),path,ans);
            }
        }
        let mut ans = 0_i32;

        dfs(root, 0, &mut ans);
        ans
    }
}

#[test]
fn t(){
    let mut i = 12;
    i ^= 1<<3;
    println!("{}",i);
}