use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution{}

impl Solution {
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(root :Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>){
            if let Some(node) = root.as_ref(){
                ans.push(node.borrow().val);
                dfs(node.borrow_mut().left.take(),ans);
                dfs(node.borrow_mut().right.take(),ans);
            }
        }
        let mut ans = vec![];
        dfs(root1,&mut ans);
        dfs(root2,&mut ans);
        ans.sort_unstable();
        ans
    }
}